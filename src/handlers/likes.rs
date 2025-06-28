use crate::database::DbPool;
use crate::repositories::likes::{like_tweet_repo, delete_like_repo, get_likes_repo};
use crate::jwt::AuthenticatedUser;
use actix_web::{HttpResponse, web, Error};
use uuid::Uuid;

/// Creates a like
pub async fn like_tweet(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let tweet_id = path.into_inner();

    // Parse user_id from JWT token
    let user_uuid = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    // Like tweet
    let like = like_tweet_repo(&pool, &user_uuid, &tweet_id)
        .map_err(|e| {
            eprintln!("Database like error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(like))
}

/// Deletes a like
pub async fn delete_like(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let tweet_id = path.into_inner();

    // Parse user_id from JWT token
    let user_uuid = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    // Delete like
    let result = delete_like_repo(&pool, &user_uuid, &tweet_id)
        .map_err(|e| {
            eprintln!("Database delete like error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Ok().json(result))
}   

/// Gets likes 
pub async fn get_likes(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let tweet_id = path.into_inner();

    // Parse user_id from JWT token
    let user_uuid = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    // Get likes
    let likes = get_likes_repo(&pool, &user_uuid, &tweet_id)
        .map_err(|e| {
            eprintln!("Database get likes error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Ok().json(likes))
}
