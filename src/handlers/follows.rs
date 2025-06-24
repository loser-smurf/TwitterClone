use crate::database::DbPool;
use crate::jwt::AuthenticatedUser;
use crate::repositories::followers::{follow_user_repo, is_followed_repo, unfollow_user_repo};
use actix_web::{Error, HttpResponse, web};
use uuid::Uuid;

/// Follow a user
pub async fn follow_user(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    // Parse user_id from JWT token
    let follower_id = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    let followed_id = path.into_inner();

    // Follow user
    let follow = follow_user_repo(&pool, &follower_id, &followed_id).map_err(|e| {
        eprintln!("Database follow error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(follow))
}

// Delete a follow
pub async fn unfollow_user(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    // Parse user_id from JWT token
    let follower_id = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    let followed_id = path.into_inner();

    // Unfollow user
    let follow = unfollow_user_repo(&pool, &follower_id, &followed_id).map_err(|e| {
        eprintln!("Database unfollow error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(follow))
}

/// Checks if a user is followed by another user
pub async fn check_follow(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    // Parse user_id from JWT token
    let follower_id = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    let followed_id = path.into_inner();

    // Check if user is followed
    let is_followed = is_followed_repo(&pool, &follower_id, &followed_id).map_err(|e| {
        eprintln!("Database check follow error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(is_followed))
}
