use crate::crypto::PasswordService;
use crate::database::DbPool;
use crate::models::users::NewUser;
use crate::repositories::users::{insert_user, find_user_by_username_or_email};
use crate::requests::users::RegisterUserRequest;

use actix_web::{HttpResponse, web, Error};

pub async fn register(
    pool: web::Data<DbPool>,
    request: web::Json<RegisterUserRequest>
) -> Result<HttpResponse, Error> {
    // Extract user registration data from the request
    let user = request.into_inner();

    // Check if a user with the same username or email already exists
    let user_exists = find_user_by_username_or_email(&pool, &user.username, &user.email)
        .map_err(|e| {
            eprintln!("Database query error: {}", e);
            actix_web::error::ErrorInternalServerError("Internal Server Error")
        })?;

    // If user exists, return HTTP 409 Conflict with a clear message
    if user_exists {
        return Ok(HttpResponse::Conflict()
            .body("A user with this username or email already exists"));
    }

    // Hash the password securely
    let password_hash = PasswordService::hash_password(&user.password)
        .map_err(|err| {
            eprintln!("Password hashing error: {:?}", err);
            actix_web::error::ErrorInternalServerError("Internal Server Error")
        })?;

    // Create a new user struct for database insertion
    let new_user = NewUser {
        username: user.username.clone(),
        email: user.email,
        password_hash,
        name: None,
        bio: None,
        avatar_url: None,
    };

    // Insert the new user into the database
    insert_user(&pool, &new_user)
        .map_err(|e| {
            eprintln!("Database insert error: {}", e);
            actix_web::error::ErrorInternalServerError("Internal Server Error")
        })?;

    // Return success message with HTTP 200 OK
    Ok(HttpResponse::Ok().body(format!("User '{}' successfully registered", user.username)))
}
