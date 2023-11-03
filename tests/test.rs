use api::fb_url_extractor::*;
use api::instagram::*;
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
    let video_id = "725308459618999".to_string();
    let url = "https://www.facebook.com/reel/725308459618999";

    if let Some(video_data) = scrap_facebook_video(url, &video_id).await {
        let hd_url = video_data.get("browser_native_hd_url").and_then(|url|{Value::as_str(url)});
        let sd_url = video_data.get("browser_native_sd_url").and_then(|url|{Value::as_str(url)});
        let image_url = video_data.pointer("/preferred_thumbnail/image/uri").and_then(|url|{Value::as_str(url)});

        println!("hd-url: {:?} \n\n sd-url: {:?} \n\n image-url: {:?}", hd_url, sd_url, image_url)
    }

    Ok(())
}

#[test]
fn facebook_url_test() {
    let test_urls = [
        "https://www.facebook.com/reel/295651653303760",
        "https://fb.watch/o4u8KCKrST/",
        "https://www.facebook.com/reel/1681069192374576",
        // Add more test URLs as needed.
    ];

    for url in &test_urls {
        match is_facebook_video_or_reel_url(url) {
            Some(id_or_shortcode) => println!(
                "Valid Facebook video/reel URL with ID or shortcode: {}",
                id_or_shortcode
            ),
            None => println!("The URL is not a valid Facebook video/reel URL."),
        }
    }
}

#[test]
fn instagram_url_test() {
    let test_urls = [
        "https://www.instagram.com/reel/ABCDEFGHIJK/",
        "https://www.instagram.com/p/ABCDEFGHIJK/",
        "https://www.instagram.com/tv/ABCDEFGHIJK/",
        "https://instagram.com/tv/ABCDEFGHIJK",
    ];

    for url in &test_urls {
        match is_instagram_reel_or_video_url(url) {
            Some(unique_id) => println!("Valid Instagram URL with ID: {}", unique_id),
            None => println!("The URL is not a valid Instagram reel or video URL."),
        }
    }
}
