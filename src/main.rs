use actix_web::{App, HttpServer, web};

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
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/api/users")
                .route("/register", web::post().to(handlers::register::register)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
