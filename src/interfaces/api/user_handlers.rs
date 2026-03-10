//! User Handlers Module
//! Implements HTTP request handlers for user authentication and management.
//!
//! Endpoints
//! - POST /api/register
//!   Request JSON:
//!     { "username": "Alice", "email": "alice@example.com", "password": "Super$ecret123" }
//!   200 OK JSON:
//!     { "create": "success", "message": "User created successfully" }
//!   400 Bad Request: "Username must contain only letters" | "Invalid email"
//!   500 Internal Server Error: "internal error: ..." o vacío
//!
//! - POST /api/login
//!   Request JSON (email o username; uno requerido):
//!     { "email": "alice@example.com", "password": "Super$ecret123" }
//!     { "username": "Alice", "password": "Super$ecret123" }
//!   400 Bad Request: "email or username is required"
//!   401 Unauthorized: credenciales inválidas o fila legacy sin hash
//!   200 OK JSON:
//!     { "token": "<JWT>" }
//!
//! JWT (HS256):
//! - Claims: { sub: "<uuid>", exp: <epoch>, iat: <epoch>, username: "<name>", roles: ["user"] }
//! - SECRET_KEY requerido (env). Expiración configurable por JWT_EXP_SECONDS.
//!
//! Seguridad:
//! - Password con bcrypt y coste configurable (BCRYPT_COST).

use crate::infrastructure::auth::jwt::{generate_token, verify_password};
use crate::infrastructure::database::surrealdb::Database;
use crate::models::entities::user::User;
use crate::models::traits::user_data_trait::UserDataTrait;
use actix_web::{web, HttpResponse, Responder};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

/// Request payload for user registration
#[derive(Deserialize)]
pub struct RegisterRequest {
    /// Username for the new account (optional if wallet is provided)
    username: Option<String>,
    /// Email for the new account (optional if wallet is provided)
    email: Option<String>,
    /// Password for the new account (optional if wallet is provided)
    password: Option<String>,
    /// Wallet address for the user (optional)
    wallet: Option<String>,
}

/// Request payload for user login (by email or username)
#[derive(Deserialize)]
pub struct LoginRequest {
    /// Email of the account (optional)
    #[serde(default)]
    email: Option<String>,
    /// Username of the account (optional)
    #[serde(default)]
    username: Option<String>,
    /// Password of the account (optional for wallet-only flow)
    #[serde(default)]
    password: Option<String>,
    /// Wallet address (optional). If provided alone, a JWT will be returned for the wallet user.
    #[serde(default)]
    wallet: Option<String>,
}

/// Response payload for successful authentication
#[derive(Serialize)]
struct AuthResponse {
    /// JWT token for the authenticated user
    token: String,
}

/// Response payload for successful registration
#[derive(Serialize)]
struct RegistrationResponse {
    /// Status of the registration
    create: String,
    /// Message detailing the result of the registration
    message: String,
}

/// Handles user registration requests
///
/// # Arguments
/// * `user_data` - JSON payload containing registration details
/// * `db` - Database connection
///
/// # Returns
/// - 200 OK if registration successful
/// - 400 Bad Request if missing required fields for either flow
/// - 500 Internal Server Error if registration fails
pub async fn register(
    user_data: web::Json<RegisterRequest>,
    db: web::Data<Database>,
) -> impl Responder {
    info!(
        "Register attempt: username={:?}, email={:?}, wallet={:?}",
        user_data.username, user_data.email, user_data.wallet
    );

    // Flow 1: Wallet registration (Demo)
    if let Some(wallet_addr) = &user_data.wallet {
        if wallet_addr.trim().is_empty() {
            return HttpResponse::BadRequest().body("Wallet address cannot be empty");
        }

        info!("Processing wallet registration for: {}", wallet_addr);
        let user = User::new_from_wallet(wallet_addr.clone());
        let log_id = user.username.clone();

        return persist_user_and_respond(&db, user, &log_id).await;
    }

    // Flow 2: Traditional registration
    let username = match &user_data.username {
        Some(u) if !u.trim().is_empty() => u.clone(),
        _ => return HttpResponse::BadRequest().body("Username is required"),
    };
    let email = match &user_data.email {
        Some(e) if !e.trim().is_empty() => e.clone(),
        _ => return HttpResponse::BadRequest().body("Email is required"),
    };
    let password = match &user_data.password {
        Some(p) if !p.trim().is_empty() => p.clone(),
        _ => return HttpResponse::BadRequest().body("Password is required"),
    };

    // Validate username: only alphabetic letters allowed
    if !username.chars().all(|c| c.is_alphabetic()) {
        warn!("Register rejected: invalid username format");
        return HttpResponse::BadRequest().body("Username must contain only letters");
    }

    // Basic email validation
    if !email.contains('@') || email.trim().len() < 5 {
        warn!("Register rejected: invalid email");
        return HttpResponse::BadRequest().body("Invalid email");
    }

    // Create new user instance using traditional flow
    let user = match User::new(username.clone(), email, password) {
        Ok(user) => {
            debug!("User::new OK for username={}", username);
            user
        }
        Err(e) => {
            error!("Register failed hashing password: {}", e);
            return HttpResponse::InternalServerError().body(format!("internal error: {}", e));
        }
    };

    let log_id = user.username.clone();
    persist_user_and_respond(&db, user, &log_id).await
}

/// Helper to persist user and return response to avoid duplication
async fn persist_user_and_respond(
    db: &web::Data<Database>,
    user: User,
    log_id: &str,
) -> HttpResponse {
    match <Database as UserDataTrait>::add_user(&db, user).await {
        Some(_) => {
            info!("Register success: id={}", log_id);
            HttpResponse::Ok().json(RegistrationResponse {
                create: "success".to_string(),
                message: "User created successfully".to_string(),
            })
        }
        None => {
            error!("Register failed persisting user: id={}", log_id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Handles user login requests
///
/// # Arguments
/// * `user_data` - JSON payload containing username and password
/// * `db` - Database connection
///
/// # Returns
/// - 200 OK with JWT token if authentication successful
/// - 401 Unauthorized if credentials invalid
/// - 500 Internal Server Error if token generation fails
pub async fn login(user_data: web::Json<LoginRequest>, db: web::Data<Database>) -> impl Responder {
    // Allow three flows:
    // 1) Wallet-only flow: { "wallet": "0x..." } -> return JWT (create user if missing)
    // 2) Traditional flow: email/username + password
    // 3) If neither present, reject

    let email = user_data.email.as_deref().filter(|s| !s.is_empty());
    let username = user_data.username.as_deref().filter(|s| !s.is_empty());
    let wallet = user_data.wallet.as_deref().filter(|s| !s.is_empty());

    info!(
        "Login attempt: email_present={}, username_present={}, wallet_present={}",
        email.is_some(),
        username.is_some(),
        wallet.is_some()
    );

    // Wallet-only flow: if wallet provided and no email/username/password, authenticate by wallet
    if let Some(w) = wallet {
        debug!("Wallet login attempt for: {}", w);

        // Try to find existing user by wallet
        let mut user = <Database as UserDataTrait>::find_user_by_wallet(&db, w).await;

        // If not found, create a new user from wallet (demo flow)
        if user.is_none() {
            info!("Wallet not found, creating new user from wallet: {}", w);
            let new_user = User::new_from_wallet(w.to_string());
            user = <Database as UserDataTrait>::add_user(&db, new_user).await;
            if user.is_none() {
                error!("Failed to persist new wallet user: {}", w);
                return HttpResponse::InternalServerError().finish();
            }
        }

        let user = user.unwrap();

        // Roles por defecto
        let roles = vec!["user".to_string()];

        // Extract user id
        let user_id_str = match &user.id {
            Some(thing) => match &thing.id {
                surrealdb::sql::Id::String(s) => s.clone(),
                surrealdb::sql::Id::Uuid(u) => u.to_string(),
                _ => thing
                    .id
                    .to_string()
                    .trim_matches('⟨')
                    .trim_matches('⟩')
                    .to_string(),
            },
            None => {
                error!("User has no id Thing set");
                return HttpResponse::InternalServerError().finish();
            }
        };

        let token = match generate_token(&user_id_str, &user.username, &roles) {
            Ok(token) => {
                info!("Wallet login success for username={}", user.username);
                token
            }
            Err(e) => {
                error!("Token generation failed: {}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

        return HttpResponse::Ok().json(AuthResponse { token });
    }

    // Traditional flow requires email or username and a password
    if email.is_none() && username.is_none() {
        warn!("Login rejected: no email, username or wallet provided");
        return HttpResponse::BadRequest().body("email, username or wallet is required");
    }

    let password = match &user_data.password {
        Some(p) if !p.trim().is_empty() => p.clone(),
        _ => {
            warn!("Login rejected: password is required for traditional flow");
            return HttpResponse::BadRequest().body("password is required");
        }
    };

    // Buscar usuario por email si está presente; si no hay resultado y vino username, intentar por username
    let mut user = if let Some(e) = email {
        debug!("Looking up user by email: {}", e);
        db.find_user_by_email(e).await
    } else {
        None
    };
    if user.is_none() {
        if let Some(u) = username {
            debug!(
                "Email lookup failed or empty; falling back to username lookup: {}",
                u
            );
            user = <Database as UserDataTrait>::find_user_by_username(&db, u).await;
        }
    }

    let user = match user {
        Some(u) => {
            debug!("User found: {}", u.username);
            u
        }
        None => {
            warn!("User not found or legacy row filtered (no password)");
            return HttpResponse::Unauthorized().finish();
        }
    };

    // Verificar que exista el hash y comparar (password es Option<String> en el modelo)
    let stored_hash = match user.password.as_deref() {
        Some(h) => h,
        None => {
            warn!(
                "User has no password hash (legacy row). username={}",
                user.username
            );
            return HttpResponse::Unauthorized().finish();
        }
    };

    let pass_ok = verify_password(&password, stored_hash);
    debug!("Password verification result: {}", pass_ok);
    if !pass_ok {
        warn!(
            "Password verification failed for username={}",
            user.username
        );
        return HttpResponse::Unauthorized().finish();
    }

    // Roles por defecto (hasta tener roles persistidos)
    let roles = vec!["user".to_string()];

    // Extraer ID puro desde Thing (evitando los brackets ⟨ ⟩ de SurrealDB)
    let user_id_str = match &user.id {
        Some(thing) => match &thing.id {
            surrealdb::sql::Id::String(s) => s.clone(),
            surrealdb::sql::Id::Uuid(u) => u.to_string(),
            _ => thing
                .id
                .to_string()
                .trim_matches('⟨')
                .trim_matches('⟩')
                .to_string(),
        },
        None => {
            error!("User has no id Thing set");
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Generar token con claims extendidos
    let token = match generate_token(&user_id_str, &user.username, &roles) {
        Ok(token) => {
            info!("Login success for username={}", user.username);
            token
        }
        Err(e) => {
            error!("Token generation failed: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(AuthResponse { token })
}
