
use flate2::read::DeflateDecoder;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use brotli_decompressor::Decompressor;

/// Decodes the given bytes based on the provided content encoding.
///
/// This function supports three types of content encodings: "br" for Brotli,
/// "gzip" for Gzip, and "deflate" for Deflate. The function will return an error
/// if an unsupported encoding is provided.
///
/// # Arguments
///
/// * `encoding`: A string slice that specifies the content encoding. Supported values are "br", "gzip", and "deflate".
/// * `bytes`: The bytes to be decoded. This is usually the raw bytes you get from a server response or any compressed source.
///
/// # Returns
///
/// * A `Result` which is either:
///     - `Ok(Vec<u8>)`: The decompressed bytes.
///     - `Err(Box<dyn std::error::Error>)`: An error that can occur during decoding or if the encoding is unsupported.
///
/// # Examples
///
/// ```
/// let encoded_data: &[u8] = ...; // Your compressed data here.
/// let decoded_data = decode_content("br", encoded_data).unwrap();
/// ```
pub fn decode_content(encoding: &str, bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let cursor = Cursor::new(bytes);

    match encoding {
        "br" => {
            let mut decompressed = Vec::new();
            Decompressor::new(cursor, 4096).read_to_end(&mut decompressed)?;
            Ok(decompressed)
        }
        "gzip" => {
            let mut decompressed = Vec::new();
            GzDecoder::new(cursor).read_to_end(&mut decompressed)?;
            Ok(decompressed)
        }
        "deflate" => {
            let mut decompressed = Vec::new();
            DeflateDecoder::new(cursor).read_to_end(&mut decompressed)?;
            Ok(decompressed)
        }
        _ => Err(Box::from("Unsupported encoding")),
    }
}

pub fn write_to_file(file_path: &str, content: &str) -> std::io::Result<()> {
    // Open the file in write mode, creating it if it doesn't exist
    let mut file = File::create(file_path)?;

    // Write the content to the file
    file.write_all(content.as_bytes())?;

    // Close the file
    file.sync_all()?;

    Ok(())
}
