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

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::models::entities::user::User;
use crate::infrastructure::auth::jwt::{verify_password, generate_token};
use crate::infrastructure::database::surrealdb::Database;
use crate::models::traits::user_data_trait::UserDataTrait;
use log::{info, debug, warn, error};

/// Request payload for user registration
#[derive(Deserialize)]
pub struct RegisterRequest {
    /// Username for the new account (letters only)
    username: String,
    /// Email for the new account
    email: String,
    /// Password for the new account
    password: String,
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
    /// Password of the account
    password: String,
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
/// * `user_data` - JSON payload containing username and password
/// * `db` - Database connection
///
/// # Returns
/// - 200 OK if registration successful
/// - 500 Internal Server Error if registration fails
pub async fn register(user_data: web::Json<RegisterRequest>, db: web::Data<Database>) -> impl Responder {
    info!("Register attempt: username={}, email={}", user_data.username, user_data.email);

    // Validate username: only alphabetic letters allowed
    if !user_data.username.chars().all(|c| c.is_alphabetic()) {
        warn!("Register rejected: invalid username format");
        return HttpResponse::BadRequest().body("Username must contain only letters");
    }

    // Basic email validation (adjust with proper validator if desired)
    if !user_data.email.contains('@') || user_data.email.trim().len() < 5 {
        warn!("Register rejected: invalid email");
        return HttpResponse::BadRequest().body("Invalid email");
    }

    // Create new user instance using the model constructor that now generates UUID and stores email
    let user = match User::new(
        user_data.username.clone(),
        user_data.email.clone(),
        user_data.password.clone(),
    ) {
        Ok(user) => {
            debug!("User::new OK for username={}", user_data.username);
            user
        },
        Err(e) => {
            error!("Register failed hashing password: {}", e);
            return HttpResponse::InternalServerError().body(format!("internal error: {}", e));
        }
    };

    // add_user devuelve Option<User> (Some si se creó, None si falló)
    match <Database as UserDataTrait>::add_user(&db, user).await {
        Some(_) => {
            info!("Register success: username={}", user_data.username);
            HttpResponse::Ok().json(RegistrationResponse {
                create: "success".to_string(),
                message: "User created successfully".to_string(),
            })
        },
        None => {
            error!("Register failed persisting user: username={}", user_data.username);
            HttpResponse::InternalServerError().finish()
        },
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
    // Determinar identificador: email o username
    let email = user_data.email.as_deref().filter(|s| !s.is_empty());
    let username = user_data.username.as_deref().filter(|s| !s.is_empty());

    info!(
        "Login attempt: email_present={}, username_present={}",
        email.is_some(),
        username.is_some()
    );

    if email.is_none() && username.is_none() {
        warn!("Login rejected: no email or username provided");
        return HttpResponse::BadRequest().body("email or username is required");
    }

    // Buscar usuario por email si está presente; si no hay resultado y vino username, intentar por username
    let mut user = if let Some(e) = email {
        debug!("Looking up user by email: {}", e);
        db.find_user_by_email(e).await
    } else {
        None
    };
    if user.is_none() {
        if let Some(u) = username {
            debug!("Email lookup failed or empty; falling back to username lookup: {}", u);
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
            warn!("User has no password hash (legacy row). username={}", user.username);
            return HttpResponse::Unauthorized().finish();
        }
    };

    let pass_ok = verify_password(&user_data.password, stored_hash);
    debug!("Password verification result: {}", pass_ok);
    if !pass_ok {
        warn!("Password verification failed for username={}", user.username);
        return HttpResponse::Unauthorized().finish();
    }

    // Roles por defecto (hasta tener roles persistidos)
    let roles = vec!["user".to_string()];

    // Extraer UUID desde Thing ("user:<uuid>" -> "<uuid>")
    let user_id_str = match &user.id {
        Some(thing) => {
            let s = thing.to_string(); // e.g., "user:783a9b75-41c2-47af-97f8-438bc623d505"
            let uuid = s.split_once(':').map(|(_, uuid)| uuid.to_string()).unwrap_or(s);
            debug!("Extracted user_id for JWT: {}", uuid);
            uuid
        }
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