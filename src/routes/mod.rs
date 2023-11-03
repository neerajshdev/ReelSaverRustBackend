use crate::fb_url_extractor::{extract_json_from_fb_doc, fetch_fb_html, find_json};
use actix_web::{get, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

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
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/get-video-data")]
pub async fn get_video_data(info: web::Json<Info>) -> impl Responder {

    let video_url = info.video_url.to_owned();
    

    let result = VideoResult {
        result : "ok".to_string(),
        thumbnail_url: "https://scontent.fdel24-1.fna.fbcdn.net/v/t51.29350-10/395705143_789011319648033_5016290835188045552_n.jpg?stp=dst-jpg_s960x960&_nc_cat=1&ccb=1-7&_nc_sid=dd673f&_nc_ohc=YluSeDdO4jwAX_WD5A2&_nc_ht=scontent.fdel24-1.fna&oh=00_AfBpxgWq5eEbX4_7T9JluvblQg2t6Hbn7J2cYV9neLu0cw&oe=65424757".to_string(), 
        video_url : "https://scontent.fdel24-1.fna.fbcdn.net/v/t50.33967-16/395013136_174947445611391_4152204066801686275_n.mp4?_nc_cat=105&ccb=1-7&_nc_sid=55d0d3&efg=eyJybHIiOjg2NCwicmxhIjo1MzAsInZlbmNvZGVfdGFnIjoiYXNpY19ocTFfc2RfcHJvZ3Jlc3NpdmUifQ%3D%3D&_nc_ohc=uCyP1IRjcyUAX-6C2UR&_nc_ht=scontent.fdel24-1.fna&oh=00_AfDYidZ_GJw5Tdni_diX7Ewm0JjNLlLxj2uhMt9E0nkNEg&oe=65425949".to_string()
    };

    web::Json(result)
}
