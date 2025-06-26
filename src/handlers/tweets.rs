use crate::database::DbPool;
use crate::jwt::AuthenticatedUser;
use crate::repositories::tweets::{
    create_reply_repo, create_retweet_repo, create_tweet_repo, delete_tweet_repo, get_replies_repo, get_tweet_repo, get_tweets_repo,
};
use crate::requests::tweets::{CreateRetweetRequest, CreateTweetRequest, TweetsQuery};
use actix_web::{Error, HttpResponse, web};
use serde_json::json;
use uuid::Uuid;

/// Creates a tweet
pub async fn create_tweet(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    tweet: web::Json<CreateTweetRequest>,
) -> Result<HttpResponse, Error> {
    // Parse user_id from JWT token
    let user_id = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    // :TODO Loads metadata from the request

    // Create tweet
    let tweet = create_tweet_repo(&pool, &user_id, &tweet.content).map_err(|e| {
        eprintln!("Database create tweet error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(tweet))
}

/// Gets tweets
pub async fn get_tweets(
    pool: web::Data<DbPool>,
    query: web::Query<TweetsQuery>,
) -> Result<HttpResponse, Error> {
    let tweets = get_tweets_repo(&pool, query.page, query.per_page).map_err(|e| {
        eprintln!("Database get tweets error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(tweets))
}

/// Gets a tweet
pub async fn get_tweet(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let tweet_id = path.into_inner();

    let tweet = get_tweet_repo(&pool, &tweet_id).map_err(|e| {
        eprintln!("Database get tweet error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(tweet))
}

/// Deletes a tweet
pub async fn delete_tweet(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let tweet_id = path.into_inner();

    // Get tweet
    let tweet = get_tweet_repo(&pool, &tweet_id).map_err(|e| {
        eprintln!("Database get tweet error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let user_uuid = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    // Check if user is the owner of the tweet
    if tweet.user_id != user_uuid {
        return Ok(HttpResponse::Forbidden().json(json!({
            "error": "You are not allowed to delete this tweet."
        })));
    }

    // Delete tweet
    let tweet = delete_tweet_repo(&pool, &tweet_id).map_err(|e| {
        eprintln!("Database delete tweet error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(tweet))
}

pub async fn reply_to_tweet(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    reply: web::Json<CreateTweetRequest>,
) -> Result<HttpResponse, Error> {
    let tweet_id = path.into_inner();

    // Fetch the tweet to which we want to reply
    let tweet = get_tweet_repo(&pool, &tweet_id).map_err(|e| {
        eprintln!("Database get tweet error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    // Check that you cannot reply to a reply (only to original tweets)
    if tweet.reply_to_id.is_some() {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You can only reply to original tweets"
        })));
    }

    // Parse user_id from JWT
    let user_uuid = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    // Create reply
    let reply = create_reply_repo(&pool, &tweet_id, &user_uuid, &reply.content).map_err(|e| {
        eprintln!("Database create reply error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(reply))
}

/// Gets replies to a tweet
pub async fn get_replies(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let tweet_id = path.into_inner();

    let replies = get_replies_repo(&pool, &tweet_id).map_err(|e| {
        eprintln!("Database get replies error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(replies))
}

/// Creates a retweet
pub async fn retweet_tweet(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    create_retweet_request: web::Json<CreateRetweetRequest>,
) -> Result<HttpResponse, Error> {
    let tweet_id = path.into_inner();

    // Parse user_id from JWT
    let user_uuid = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    // Create retweet
    let retweet = create_retweet_repo(&pool, &tweet_id, &user_uuid, create_retweet_request.content.clone()).map_err(|e| {
        eprintln!("Database create retweet error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(retweet))
}