use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello World!"}))
            .service(handlers::tweet_index)
            .service(handlers::tweet_create)
            .service(handlers::tweet_show)
            .service(handlers::tweet_update)
            .service(handlers::tweet_delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
