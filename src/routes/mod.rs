use crate::fb_url_extractor::get_fb_video_data;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct VideoResult {
    result: String,
    thumbnail_url: String,
    video_url: String,
}

#[derive(Deserialize)]
pub struct Info {
    video_url: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    println!("greeting..");
    HttpResponse::Ok().body("Hello, world!")
}


#[get("/get-video-data")]
pub async fn get_video_data(info: web::Json<Info>) -> impl Responder {
    let video_url = info.video_url.to_owned();

    let video_data = if is_facebook_url(&video_url) {
        get_fb_video_data(&video_url).await.and_then(|data| {
            let hd_url = data
                .get("browser_native_hd_url")
                .and_then(|url| Value::as_str(url));
            let sd_url = data
                .get("browser_native_sd_url")
                .and_then(|url| Value::as_str(url));
            let image_url = data
                .pointer("/preferred_thumbnail/image/uri")
                .and_then(|url| Value::as_str(url));

            let video_url = hd_url.or(sd_url);

            video_url.and_then(|video_url| {
                image_url.map(|image_url| VideoResult {
                    result: "ok".to_string(),
                    video_url: video_url.to_string(),
                    thumbnail_url: image_url.to_string(),
                })
            })
        })
    } else {
        None
    };

    match video_data {
        Some(response) => web::Json(response),
        None => web::Json(VideoResult {
            result: "none".to_string(),
            thumbnail_url: "".to_string(),
            video_url: "".to_string(),
        }),
    }
}

fn is_facebook_url(url: &str) -> bool {
    match url::Url::parse(url) {
        Ok(url) => {
            if let Some(host) = url.host_str() {
                host.ends_with("facebook.com")
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
