use std::net::SocketAddr;
use std::error::Error;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{header, Body,Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use hyper_tls::HttpsConnector;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub id: String,
    pub height: u32,
    pub version: u32,
    pub timestamp: u32,
    pub tx_count: u32,
    pub size: u32,
    pub weight: u32,
    pub merkle_root: String,
    pub mediantime: u32,
    pub nonce: u32,
    pub bits: u32,
    pub difficulty: u32
}

const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {

    let https = HttpsConnector::new();
    let client = hyper::Client::builder()
    .build::<_, hyper::Body>(https);

    let request_url = format!("https://blockstream.info/api/blocks/{block}", block = "0");
    println!("{}",request_url);
    let req = Request::builder()
        .method(Method::GET)
        .uri(request_url)
        .header("user-agent", "the-awesome-agent/007")
        .body(hyper::Body::from(""))?;

    // Pass our request builder object to our client.
    let resp = client.request(req).await?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(handle_request))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
