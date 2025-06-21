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
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
