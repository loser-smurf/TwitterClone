use crate::database::DbPool;
use crate::jwt::AuthenticatedUser;
use crate::repositories::tweets::{create_tweet_repo, get_tweet_repo, get_tweets_repo};
use crate::requests::tweets::{CreateTweetRequest, TweetsQuery};
use actix_web::{Error, HttpResponse, web};
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
