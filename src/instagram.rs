use actix_web::http::header;
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONTENT_TYPE,
    DNT, ORIGIN, REFERER, USER_AGENT,
};

use regex::Regex;


pub fn is_instagram_reel_or_video_url(url: &str) -> Option<String> {
    // Define a regex pattern for Instagram reels or videos.
    let reel_video_regex = Regex::new(
        r"(?x)                                          # Enable verbose mode (comments and whitespace).
        https?://(www\.)?instagram\.com/               # Match the beginning of an Instagram URL.
        (?:p|tv|reel)/                                 # Match 'p', 'tv', or 'reel' indicating the type of content.
        ([\w-]+)/?                                     # Capture the unique identifier for the reel or video.
        "
    ).unwrap();

    // Check if the pattern matches and extract the unique identifier.
    reel_video_regex.captures(url).and_then(|caps| {
        caps.get(2).map(|m| m.as_str().to_string())
    })
}



pub async fn fetch_instagram_video(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();

    headers.insert(
        USER_AGENT, 
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36")
    );
    headers.insert(
        ORIGIN,
        HeaderValue::from_static("https://www.instagram.com"),
    );

    headers.insert(
        header::COOKIE, 
        HeaderValue::from_static(
            "sessionid=19755146960%3Ah0z2IcJdFIDh7g%3A26%3AAYdsypG0nIE2_apEiHD0g_T0EO5auEUtDL4lcpKqFg; ds_user_id=19755146960"
        )
    );

    // headers.insert(
    //     REFERER,
    //     HeaderValue::from_static(
    //         "https://www.instagram.com/reel/CwevrKxNXRt/?utm_source=ig_web_copy_link",
    //     ),
    // );
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    // headers.insert(
    //     ACCEPT_ENCODING,
    //     HeaderValue::from_static("gzip, deflate, br"),
    // );
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.9,hi;q=0.8"),
    );
    // headers.insert(
    //     CONTENT_TYPE,
    //     HeaderValue::from_static("application/json"),
    // );
    headers.insert(DNT, HeaderValue::from_static("1"));
    headers.insert("dpr", HeaderValue::from_static("1"));
    headers.insert("viewport-width", HeaderValue::from_static("825"));

    // Custom headers
    headers.insert(
        HeaderName::from_static("sec-ch-ua"),
        HeaderValue::from_static(
            "\"Chromium\";v=\"118\", \"Google Chrome\";v=\"118\", \"Not;A Brand\";v=\"99\"",
        ),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua-mobile"),
        HeaderValue::from_static("?0"),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua-model"),
        HeaderValue::from_static(""),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua-platform"),
        HeaderValue::from_static("Windows"),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua-platform-version"),
        HeaderValue::from_static("10.0.0"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-dest"),
        HeaderValue::from_static("empty"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-mode"),
        HeaderValue::from_static("cors"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-site"),
        HeaderValue::from_static("same-origin"),
    );

    // let mut params = HashMap::new();
    // Yahan aapko koi parameters add karne hain toh woh kariye, varna ise chhod dijiye.

    let resp: reqwest::Response = client.get(url).headers(headers).send().await?;
    let result = resp.text().await;
    if let Ok(content) = result {
        Ok(content)
    } else {
        Err(Box::from("Something went wrong"))
    }
}
