use crate::crypto::PasswordService;
use crate::database::DbPool;
use crate::jwt::{AuthenticatedUser, create_jwt};
use crate::models::users::NewUser;
use crate::repositories::users::{
    does_user_exist, find_user_by_id, find_user_by_username_or_email, insert_user,
};
use crate::requests::users::{LoginRequest, RegisterRequest};
use uuid::Uuid;

use actix_web::{Error, HttpResponse, web};

/// Handles user registration
pub async fn register(
    pool: web::Data<DbPool>,
    request: web::Json<RegisterRequest>,
) -> Result<HttpResponse, Error> {
    // Extract user registration data from the request
    let user = request.into_inner();

    // Check if a user with the same username or email already exists
    let user_exists = does_user_exist(&pool, &user.username, &user.email).map_err(|e| {
        eprintln!("Database query error: {}", e);
        actix_web::error::ErrorInternalServerError("Internal Server Error")
    })?;

    // If user exists, return HTTP 409 Conflict with a clear message
    if user_exists {
        return Ok(
            HttpResponse::Conflict().body("A user with this username or email already exists")
        );
    }

    // Hash the password securely
    let password_hash = PasswordService::hash_password(&user.password).map_err(|err| {
        eprintln!("Password hashing error: {:?}", err);
        actix_web::error::ErrorInternalServerError("Internal Server Error")
    })?;

    // Create a new user struct for database insertion
    let new_user = NewUser {
        username: user.username.clone(),
        email: user.email,
        password_hash,
        name: user.name,
        bio: user.bio,
        avatar_url: user.avatar_url,
    };

    // Insert the new user into the database
    insert_user(&pool, &new_user).map_err(|e| {
        eprintln!("Database insert error: {}", e);
        actix_web::error::ErrorInternalServerError("Internal Server Error")
    })?;

    // Return success message with HTTP 200 OK
    Ok(HttpResponse::Ok().body(format!("User '{}' successfully registered", user.username)))
}

/// Handles user login and JWT token generation
pub async fn login(
    pool: web::Data<DbPool>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    // Extract login data from the request JSON payload
    let login_data = request.into_inner();

    // Attempt to find the user by username or email in the database
    let user =
        find_user_by_username_or_email(&pool, &login_data.username_or_email).map_err(|e| {
            eprintln!("DB query error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if let Some(user) = user {
        // Verify the provided password against the stored password hash
        let valid = PasswordService::verify_password(&login_data.password, &user.password_hash)
            .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;

        if valid {
            // Generate a JWT token containing the user ID as the subject
            let token = create_jwt(&user.id.to_string())
                .map_err(|_| actix_web::error::ErrorInternalServerError("Token creation error"))?;

            // Return the token as a JSON response body
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(format!(r#"{{"token":"{}"}}"#, token)));
        }
    }

    // Return Unauthorized error if user not found or password is invalid
    Err(actix_web::error::ErrorUnauthorized(
        "Invalid username/email or password",
    ))
}

/// Example protected endpoint which requires a valid JWT token.
/// Returns the user ID extracted from the token.
pub async fn protected_route(user: AuthenticatedUser) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(format!("Welcome, user with id: {}", user.user_id)))
}

pub async fn get_current_user(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    // Parse user_id from string to UUID
    let user_id = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    // Find user in database by ID from JWT token
    let db_user = find_user_by_id(&pool, &user_id)
        .map_err(|e| {
            eprintln!("Database query error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    // Return user information (excluding sensitive data like password_hash)
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(format!(
            r#"{{"id":"{}","username":"{}","email":"{}","name":{},"bio":{},"avatar_url":{},"created_at":"{}"}}"#,
            db_user.id,
            db_user.username,
            db_user.email,
            db_user.name.as_deref().map(|s| format!("\"{}\"", s)).unwrap_or_else(|| "null".to_string()),
            db_user.bio.as_deref().map(|s| format!("\"{}\"", s)).unwrap_or_else(|| "null".to_string()),
            db_user.avatar_url.as_deref().map(|s| format!("\"{}\"", s)).unwrap_or_else(|| "null".to_string()),
            db_user.created_at
        )))
}
