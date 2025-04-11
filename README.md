# ReelSaver Rust Backend

A Rust-powered backend service that allows users to extract and download videos from social media platforms like Facebook. It provides a simple HTTP API to retrieve video URLs and thumbnail images.

## Features

- Extract direct video URLs from Facebook links
- Retrieve thumbnail image URLs
- RESTful API using Actix-web framework
- Supports various compression formats (Brotli, Gzip, Deflate)
- Containerized with Docker for easy deployment

## Getting Started

### Prerequisites

- Rust 1.54 or higher
- Cargo package manager

### Installation

Clone the repository:

```bash
git clone https://github.com/neerajshdev/ReelSaverRustBackend.git
cd ReelSaverRustBackend
```

Build the project:

```bash
cargo build --release
```

Run the server:

```bash
cargo run --release
```

By default, the server will run on `0.0.0.0:8000`.

### Docker

You can also run the application using Docker:

```bash
docker build -t reelsaver-backend .
docker run -p 8000:8000 reelsaver-backend
```

Or using Docker Compose:

```bash
docker-compose up -d
```

## API Endpoints

### `GET /`

A simple health check endpoint that returns "Hello, world!".

### `GET /get-video-data`

Extract video data from a social media URL.

**Request Body:**

```json
{
  "video_url": "https://www.facebook.com/example/videos/123456789"
}
```

**Response:**

```json
{
  "result": "ok",
  "thumbnail_url": "https://example.com/thumbnail.jpg",
  "video_url": "https://example.com/video.mp4"
}
```

If no video data is found, the `result` field will be "none".

## Development

### Project Structure

- `src/main.rs` - Application entry point
- `src/routes/` - API route definitions
- `src/fb_url_extractor.rs` - Facebook video extraction logic
- `src/instagram.rs` - Instagram video extraction logic
- `src/util.rs` - Utility functions for compression and file operations
