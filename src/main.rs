use actix_web::{App, HttpServer, web};

mod jwt;
mod crypto;
mod database;
mod handlers;
mod models;
mod repositories;
mod requests;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = database::create_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/auth/")
                    .route("/register", web::post().to(handlers::auth::register))
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("/protected", web::post().to(handlers::auth::protected_route))
                    .route("/me", web::get().to(handlers::auth::get_current_user))
            )
            .service(
                web::scope("/users/")
                    .route("", web::get().to(handlers::users::list_users))
                    .route("/{id}", web::get().to(handlers::users::get_user))
                    .route("/{id}", web::patch().to(handlers::users::update_user))
                    .route("/{id}", web::delete().to(handlers::users::delete_user))
                    .route("/{id}/followers", web::get().to(handlers::users::get_followers))
                    .route("/{id}/following", web::get().to(handlers::users::get_followings))
            )
            .service(
                web::scope("/follows/")
                    .route("/{id}", web::post().to(handlers::follows::follow_user))
                    .route("/{id}", web::delete().to(handlers::follows::unfollow_user))
                    .route("/{id}", web::get().to(handlers::follows::check_follow))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
