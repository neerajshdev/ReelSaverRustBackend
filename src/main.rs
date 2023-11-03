mod fb_url_extractor;
mod instagram;
mod routes;
mod util;

use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::get_video_data)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
