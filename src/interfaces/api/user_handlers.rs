//! User Handlers Module
//! Implements HTTP request handlers for user authentication and management.

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::models::entities::user::User;
use crate::infrastructure::auth::jwt::{verify_password, generate_token};
use crate::infrastructure::database::surrealdb::Database;
use crate::models::traits::user_data_trait::UserDataTrait;

/// Request payload for user registration
#[derive(Deserialize)]
pub struct RegisterRequest {
    /// Username for the new account
    username: String,
    /// Password for the new account
    password: String,
}

/// Request payload for user login
#[derive(Deserialize)]
pub struct LoginRequest {
    /// Username of the account
    username: String,
    /// Password of the account
    password: String,
}

/// Response payload for successful authentication
#[derive(Serialize)]
struct AuthResponse {
    /// JWT token for the authenticated user
    token: String,
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
    // Create new user instance with hashed password
    let user = match User::new(user_data.username.clone(), user_data.password.clone()) {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Attempt to store the new user in database
    match <Database as UserDataTrait>::add_user(&db, user).await {
        Some(_) => HttpResponse::Ok().json("User registered successfully"),
        None => HttpResponse::InternalServerError().finish(),
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
    // Verify user exists in database
    let user = match <Database as UserDataTrait>::find_user_by_username(&db, &user_data.username).await {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().finish(),
    };

    // Verify password hash matches
    if !verify_password(&user_data.password, &user.password) {
        return HttpResponse::Unauthorized().finish();
    }

    // Generate JWT token for authenticated user
    let token = match user.id {
        Some(ref id) => match generate_token(id) {
            Ok(token) => token,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        },
        None => return HttpResponse::InternalServerError().finish(),
    };

    // Return successful authentication response with token
    HttpResponse::Ok().json(AuthResponse { token })
}