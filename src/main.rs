use anyhow::{anyhow, Result};
use hyper::{Body, Response, Server};
use routerify::{Router, RouterService};
use std::{net::SocketAddr, str::FromStr};

#[tokio::main]
pub async fn main() -> Result<()> {
    let router = Router::builder()
        .any(|_req| async move {
            let body = std::env::var("BODY").unwrap_or(String::new());
            let status = std::env::var("STATUS").unwrap_or(String::from("200"));
            let content_type = std::env::var("CONTENT_TYPE").unwrap_or(String::from("application/json"));
            Response::builder()
                .header(hyper::header::CONTENT_TYPE, content_type)
                .status(status.as_str())
                .body(Body::from(String::from(body)))
        })
        .build()
        .unwrap();
    let rs = match RouterService::new(router) {
        Err(e) => Err(anyhow!(e)),
        Ok(rs) => Ok(rs),
    }?;
    let host = std::env::var("HOST").unwrap_or(String::from("127.0.0.1"));
    let port: u32 = std::env::var("PORT").unwrap_or(String::from("3000")).parse()?;
    let addr = SocketAddr::from_str(&format!("{}:{}", host, port))?;
    println!("Server started listeninig on {}", addr);
    if let Err(e) = Server::bind(&addr).serve(rs).await {
        eprintln!("Server error: {}", e);
    }
    Ok(())
}
