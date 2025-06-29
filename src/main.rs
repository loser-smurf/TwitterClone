use crate::storage::S3Storage;
use actix_web::{App, HttpServer, web};
use aws_config::BehaviorVersion;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Region;
use std::env;

mod crypto;
mod database;
mod handlers;
mod jwt;
mod models;
mod repositories;
mod requests;
mod schema;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let region_provider = RegionProviderChain::default_provider().or_else(Region::new(
        env::var("AWS_REGION").unwrap_or_else(|_| "eu-north-1".to_string()),
    ));

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = Client::new(&config);
    let baucket_name = env::var("AWS_BUCKET_NAME").unwrap_or_else(|_| "file-storage".to_string());
    let storage_s3 = S3Storage::new(client, baucket_name);

    let pool = database::create_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(storage_s3.clone()))

            // Auth endpoints
            .service(
                web::scope("/auth/")
                    .route("/register", web::post().to(handlers::auth::register))
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("/protected", web::post().to(handlers::auth::protected_route))
                    .route("/me", web::get().to(handlers::auth::get_current_user))
            )

            // User endpoints
            .service(
                web::scope("/users/")
                    .route("", web::get().to(handlers::users::list_users))
                    .route("/{id}", web::get().to(handlers::users::get_user))
                    .route("/{id}", web::patch().to(handlers::users::update_user))
                    .route("/{id}", web::delete().to(handlers::users::delete_user))
                    .route("/{id}/followers", web::get().to(handlers::users::get_followers))
                    .route("/{id}/following", web::get().to(handlers::users::get_followings))
            )

            // Follow endpoints
            .service(
                web::scope("/follows/")
                    .route("/{id}", web::post().to(handlers::follows::follow_user))
                    .route("/{id}", web::delete().to(handlers::follows::unfollow_user))
                    .route("/{id}", web::get().to(handlers::follows::check_follow))
            )

            // Tweet endpoints
            .service(
                web::scope("/tweets/")
                    .route("", web::post().to(handlers::tweets::create_tweet))
                    .route("", web::get().to(handlers::tweets::get_tweets))
                    .route("/{id}", web::get().to(handlers::tweets::get_tweet))
                    .route("", web::delete().to(handlers::tweets::delete_tweet))
                    .route("/{id}/reply", web::post().to(handlers::tweets::reply_to_tweet))
                    .route("/{id}/replies", web::get().to(handlers::tweets::get_replies))
                    .route("/{id}/retweet", web::post().to(handlers::tweets::retweet_tweet))
                    .route("/{id}/like", web::post().to(handlers::likes::like_tweet))
                    .route("/{id}/like", web::delete().to(handlers::likes::delete_like))
                    .route("/{id}/likes", web::get().to(handlers::likes::get_likes))
            )

            // Media endpoints
            .service(
                web::scope("/media/")
                    .route("/upload", web::post().to(handlers::media::upload_media))
            )

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}