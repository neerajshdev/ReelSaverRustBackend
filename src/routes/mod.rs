use std::result;

use crate::{
    fb_url_extractor::{
        extract_json_from_fb_doc, fetch_fb_html, find_json, is_facebook_video_or_reel_url,
        scrap_facebook_video,
    },
    instagram::is_instagram_reel_or_video_url,
};
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

    let some: Option<VideoResult> = if let Some(id) = is_facebook_video_or_reel_url(&video_url) {
        // println!("given url {} is facebook url with id {}", &video_url, &id);

        if let Some(video_data) = scrap_facebook_video(&video_url, &id).await {
            println!("{:?}", serde_json::to_string_pretty(&video_data));

            let hd_url = video_data
                .get("browser_native_hd_url")
                .and_then(|url| Value::as_str(url));
            let sd_url = video_data
                .get("browser_native_sd_url")
                .and_then(|url| Value::as_str(url));
            let image_url = video_data
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
        } else {
           
            None
        }
    } else if let _id = is_instagram_reel_or_video_url(&video_url) {
        None
    } else {
        None
    };

    if some.is_none() {
        println!("could not scrap {}", video_url);
    }

    match some {
        Some(response) => web::Json(response),
        None => web::Json(VideoResult {
            result: "none".to_string(),
            thumbnail_url: "".to_string(),
            video_url: "".to_string(),
        }),
    }
}
