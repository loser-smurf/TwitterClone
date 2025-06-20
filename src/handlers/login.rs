use actix_web::{HttpResponse, web, Error};
use crate::crypto::PasswordService;
use crate::database::DbPool;
use crate::repositories::users::find_user_by_username_or_email;
use crate::requests::users::LoginRequest; 
use crate::jwt::{create_jwt, AuthenticatedUser}; 

pub async fn login(
    pool: web::Data<DbPool>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    // Extract login data from the request JSON payload
    let login_data = request.into_inner();

    // Attempt to find the user by username or email in the database
    let user = find_user_by_username_or_email(&pool, &login_data.username_or_email)
        .map_err(|e| {
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
    Err(actix_web::error::ErrorUnauthorized("Invalid username/email or password"))
}

/// Example protected endpoint which requires a valid JWT token.
/// Returns the user ID extracted from the token.
pub async fn protected_route(user: AuthenticatedUser) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(format!("Welcome, user with id: {}", user.user_id)))
}
