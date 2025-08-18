use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::models::entities::user::User;
use crate::infrastructure::auth::jwt::{verify_password, generate_token};
use crate::infrastructure::database::surrealdb::Database;
use crate::models::traits::user_data_trait::UserDataTrait;

// Struct for user registration request
#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

// Struct for user login request
#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

// Struct for authentication response
#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

// Handler for user registration
pub async fn register(user_data: web::Json<RegisterRequest>, db: web::Data<Database>) -> impl Responder {
    // Create a new User instance
    let user = match User::new(user_data.username.clone(), user_data.password.clone()) {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Attempt to add the new user to the database
    match <Database as UserDataTrait>::add_user(&db, user).await {
        Some(_) => HttpResponse::Ok().json("User registered successfully"),
        None => HttpResponse::InternalServerError().finish(),
    }
}

// Handler for user login
pub async fn login(user_data: web::Json<LoginRequest>, db: web::Data<Database>) -> impl Responder {
    // Attempt to find the user by username
    let user = match <Database as UserDataTrait>::find_user_by_username(&db, &user_data.username).await {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().finish(),
    };

    // Verify the provided password
    if !verify_password(&user_data.password, &user.password) {
        return HttpResponse::Unauthorized().finish();
    }

    // Generate a JWT token for the authenticated user
    let token = match user.id {
        Some(ref id) => match generate_token(id) {
            Ok(token) => token,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        },
        None => return HttpResponse::InternalServerError().finish(),
    };

    // Return the token in the response
    HttpResponse::Ok().json(AuthResponse { token })
}