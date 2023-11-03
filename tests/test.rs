
use api::fb_url_extractor::*;
use api::instagram::fetch_instagram_video;
use api::util::*;
use serde_json::Value;



#[tokio::test]
async fn simple_test() -> Result<(), std::io::Error> {
    let url = "https://www.instagram.com/reel/CwewKrxNXRT/?__a=1&__d=dis";

    if let Ok(content) = fetch_instagram_video(url).await {
        write_to_file("instagram.html", &content).unwrap();
    }

    Ok(())
}

#[tokio::test]
async fn facebook_video_scrap_test() -> Result<(), std::io::Error> {

    let video_id = "884367969807005".to_string();
    let url = "https://www.facebook.com/reel/884367969807005";
    let mut count = 1;

    if let Ok(html_content) = fetch_fb_html(&url).await {
        for json in extract_json_from_fb_doc(&html_content) {
            if let Some(inner_json) = find_json(&json, "video") {
                if let Value::Object(map) = inner_json.to_owned() {
                    if let Some(id) = map.get("id") {
                        if let Value::String(id) = id {
                            if *id == video_id {
                                if let Some(value) = map.get("creation_story") {
                                    
                                    if let Value::Object(creation_story) = value{
                                        if creation_story.contains_key("short_form_video_context") {
                                            println!("inner json {}: {}", count, inner_json.to_string());
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
