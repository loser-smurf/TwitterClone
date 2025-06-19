use actix_web::{HttpServer, App, web};

mod database;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = database::create_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("")
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
