
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod state;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(state::AppStateWithCounter {
        counter: Mutex::new(0),
    });

    let app = move || {
        // move counter into the closure
        App::new()
            // Note: using app_data instead of data
            .app_data(
                // Json extractor configuration for this resource.
                web::JsonConfig::default()
                    .limit(4096) // Limit request payload size
                    .content_type(|mime| {mime == mime::APPLICATION_JSON}))
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(handlers::index))
    };

    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
