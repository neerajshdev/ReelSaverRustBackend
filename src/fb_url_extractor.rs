use crate::util::decode_content;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, USER_AGENT};
use serde_json::Map;
use serde_json::{Result as SerdeResult, Value};
use url::Url;

pub fn is_facebook_video_or_reel_url(url: &str) -> Option<String> {
    // Define a regex pattern that matches Facebook video or reel URLs.
    let fb_regex = Regex::new(
        r"(?x)                                    # Enable verbose mode for this regex.
        https?://(?:www\.|web\.|m\.)?facebook\.com/ # Match the start of a Facebook domain.
        (?:                                       # Start non-capturing group for the different URL types.
            watch/\?v=                            # Match 'watch' URLs.
            |                                    # OR
            reel/                                # Match 'reel' URLs.
            |                                    # OR
            .*videos.*vb\.\d+/                   # Match any characters followed by 'videos' and 'vb.<number>/'.
        )                                         # End non-capturing group.
        (?P<id>[0-9]+)                           # Capture the numeric ID named 'id'.
        |                                        # OR
        https?://fb\.watch/(?P<shortcode>[A-Za-z0-9_\-]+)/ # Match 'fb.watch' short URLs and capture the shortcode.
        "
    ).unwrap();

    // Attempt to find a match and extract the video or reel ID.
    fb_regex.captures(url).and_then(|caps| {
        caps.name("id")
            .or_else(|| caps.name("shortcode"))
            .map(|m| m.as_str().to_string())
    })
}

pub async fn scrap_facebook_video(url: &str, id: &str) -> Option<Value> {
    let fetched_html = fetch_fb_html(url).await.ok()?;
    let video_id = id.to_string();

    // println!("fb_html from {} \n {}", url, fetched_html);

    extract_json_from_fb_doc(&fetched_html).into_iter().find_map(|json|{
        match find_json(&json, "video") {
            Some(Value::Object(map)) if map.get("id") == Some(&Value::String(video_id.clone())) => {
                map.get("creation_story")
                .and_then(|cs| cs.get("short_form_video_context"))
                .and_then(|vc| vc.get("playback_video"))
                .cloned()
            }
            _ => None
        }
    })
}

/**a
 * Find all Url from facebook video urls
 */
pub async fn find_urls(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let html_content = fetch_fb_html(url).await?;
    let script_data = extract_json_from_fb_doc(&html_content);

    println!("{}", html_content);

    print!("\n\n {:#?}", script_data);

    Ok(script_data
        .into_iter()
        .filter_map(|script| extract_urls_from_json(&script))
        .flatten()
        .collect())
}

/**
 * Download the html content from the facebook link to video
 */

pub async fn fetch_fb_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Set headers
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );

    headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));

    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert("Cookie", HeaderValue::from_static(""));
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("en-US,en;q=0.9"),
    );

    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"));

    headers.insert("Viewport-Width", HeaderValue::from_static("1064"));
    headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
    headers.insert("Sec-Fetch-User", HeaderValue::from_static("?1"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
    headers.insert(
        "Sec-Ch-Ua-Platform-Version",
        HeaderValue::from_static("10.0.0"),
    );
    headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static("Windows"));
    headers.insert("Sec-Ch-Ua-Model", HeaderValue::from_static(""));
    headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
    headers.insert("Sec-Ch-Ua-Full-Version-List", HeaderValue::from_static("\"Chromium\";v=\"118.0.5993.89\", \"Google Chrome\";v=\"118.0.5993.89\", \"Not=A?Brand\";v=\"99.0.0.0\""));
    headers.insert(
        "Sec-Ch-Ua",
        HeaderValue::from_static(
            "\"Chromium\";v=\"118\", \"Google Chrome\";v=\"118\", \"Not=A?Brand\";v=\"99\"",
        ),
    );

    headers.insert(
        "Sec-Ch-Prefers-Color-Scheme",
        HeaderValue::from_static("light"),
    );

    headers.insert("Dpr", HeaderValue::from_static("1"));
    headers.insert("Cache-Control", HeaderValue::from_static("max-age=0"));

    let response = client.get(url).headers(headers).send().await;
    println!("fb_request respone {:?}", response);
    let response = response?;

    // Handle decompression based on the content-encoding
    let res_headers = response.headers().clone();
    let content_encoding = res_headers
        .get("content-encoding")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    match decode_content(&content_encoding, &response.bytes().await?) {
        Ok(body) => Ok(String::from_utf8(body).unwrap()),
        Err(e) => Err(Box::from("Error decoding content-encoding")),
    }
}

pub fn extract_json_from_fb_doc(html_content: &str) -> Vec<String> {
    let mut jsons = vec![];
    let re = Regex::new(r#"<script type="application/json" data-content-len=".*?" data-sjs>[\s\n]*(\{.*?\})[\s\n]*</script>"#).unwrap();

    for cap in re.captures_iter(html_content) {
        let json_str = &cap[1];
        let parsed: Result<Value, _> = serde_json::from_str(json_str);

        if let Ok(json_value) = parsed {
            jsons.push(json_value.to_string())
        }
    }

    jsons
}

/** find a key value from a json_content */
pub fn find_json(json_content: &str, key: &str) -> Option<Value> {
    let result: SerdeResult<Value> = serde_json::from_str(json_content);
    match result {
        Ok(json_obj) => find_value_by_key(&json_obj, key),
        Err(_) => None,
    }
}

/** recusive method to search for a key inside json */
fn find_value_by_key(json_value: &Value, key: &str) -> Option<Value> {
    match json_value {
        Value::Object(map) => {
            // Check if the key exists in the object
            if let Some(value) = map.get(key) {
                return Some(value.to_owned());
            }

            // Otherwise, search deeper into each nested object
            for (_k, v) in map {
                if let Some(value) = find_value_by_key(v, key) {
                    return Some(value);
                }
            }
        }
        Value::Array(arr) => {
            // Search each item in the array
            for item in arr {
                if let Some(value) = find_value_by_key(item, key) {
                    return Some(value);
                }
            }
        }
        // Other types of Value can't contain nested objects, so we skip them
        _ => {}
    }

    None
}

fn find_key(json_obj: &Map<String, Value>, key: &str) -> Option<Value> {
    for (token, value) in json_obj {
        if token == key {
            return Some(value.clone());
        }

        if let Value::Object(json) = value {
            return find_key(json, key);
        }

        if let Value::Array(json_array) = value {
            return find_key_arr(json_array, key);
        }
    }
    None
}

fn find_key_arr(json_array: &Vec<Value>, key: &str) -> Option<Value> {
    for elem in json_array {
        if let Value::Object(json) = elem {
            return find_key(json, key);
        }

        if let Value::Array(json_array) = elem {
            return find_key_arr(json_array, key);
        }
    }

    None
}

fn extract_urls_from_json(json_content: &str) -> Option<Vec<String>> {
    // First, parse the string as a JSON string
    let mut urls: Vec<String> = vec![];
    let value: SerdeResult<Value> = serde_json::from_str(json_content);
    match value {
        Ok(json) => {
            parse_json(&json, &mut urls);
            Some(urls)
        }

        Err(e) => None,
    }
}

fn parse_json(json: &Value, urls: &mut Vec<String>) {
    match json {
        Value::String(s) => {
            if is_valid_url(s) && Url::parse(s).unwrap().path().ends_with(".mp4") {
                urls.push(s.to_string());
            }
        }
        Value::Array(arr) => {
            for val in arr {
                parse_json(val, urls);
            }
        }
        Value::Object(obj) => {
            for (key, val) in obj {
                if key == "video" {
                    println!(" \n {:#?} \n", val);
                }
                parse_json(val, urls);
            }
        }
        _ => {}
    }
}

fn is_valid_url(s: &str) -> bool {
    s.parse::<Url>().is_ok()
}
