use clap::Parser;
use std::net::SocketAddr;
use tokio_util::io::ReaderStream;

use axum:: {
    body::StreamBody,
    http::{header, HeaderValue, StatusCode, HeaderMap},
    extract::State,
    extract::ConnectInfo,
    routing::get,
    Router,
    response::IntoResponse,
};

use ngrok::prelude::*;

#[derive(Parser)]
struct Options {
    // first option is the path
    path: std::path::PathBuf
}

#[derive(Clone, Default)]
struct StaticServerConfig {
    pub(crate) path: std::path::PathBuf,
 }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // get options from path
    let opt = Options::parse();

    // build our application with a single route
    let app = Router::new().route("/", get(serve))
    .fallback(serve)
    .with_state(StaticServerConfig{
        path: opt.path.clone()
    });

    let tun = ngrok::Session::builder()
        // Read the token from the NGROK_AUTHTOKEN environment variable
        .authtoken_from_env()
        // Connect the ngrok session
        .connect()
        .await?
        // Start a tunnel with an HTTP edge
        .http_endpoint()
        .listen()
        .await?;

    println!("Serving {:?} on URL: {:?}", opt.path.canonicalize().ok().unwrap().to_string_lossy(), tun.url());

    // Instead of binding a local port like so:
    // axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
    // Run it with an ngrok tunnel instead!
    axum::Server::builder(tun)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}

async fn serve(State(cfg): State<StaticServerConfig>, ConnectInfo(remote_addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    println!("Sending file to {remote_addr:?}");
    let path = cfg.path;
    let file = match tokio::fs::File::open(&path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };


    let file_name = path.file_name().unwrap();

    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/binary"));
    headers.insert(header::CONTENT_DISPOSITION, HeaderValue::from_str(format!("attachment; filename={:?}", file_name).as_str()).unwrap());

    Ok((headers, body))
}