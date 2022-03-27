use hyper::{
    Error,
    Body,
    Request,
    Response,
    Server,
    service::{make_service_fn, service_fn},
};
use std::{process, net::SocketAddr, str::FromStr};

async fn any_handler(_req: Request<Body>) -> Result<Response<Body>, Error> {
    let body = std::env::var("BODY").unwrap_or(String::new());
    let status = std::env::var("STATUS").unwrap_or(String::from("200"));
    let content_type = std::env::var("CONTENT_TYPE").unwrap_or(String::from("application/json"));

    Ok(Response::builder()
        .header(hyper::header::CONTENT_TYPE, content_type)
        .status(status.as_str())
        .body(Body::from(String::from(body)))
        .unwrap())
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tokio::spawn(async move {
        use tokio::signal::unix::{signal, SignalKind};
        let mut hup = signal(SignalKind::hangup()).unwrap();
        let mut int = signal(SignalKind::interrupt()).unwrap();
        let mut quit = signal(SignalKind::quit()).unwrap();
        let mut term = signal(SignalKind::terminate()).unwrap();

        tokio::select! {
            _ = hup.recv() => println!("Recieved SIGHUP!"),
            _ = int.recv() => println!("Recieved SIGINT!"),
            _ = quit.recv() => println!("Recieved SIGQUIT!"),
            _ = term.recv() => println!("Recieved SIGTERM!"),
        }
        println!("Good Bye!");
        process::exit(0);
    });

    let host = std::env::var("HOST").unwrap_or(String::from("0.0.0.0"));
    let port: u32 = std::env::var("PORT").unwrap_or(String::from("3000")).parse()?;
    let addr = SocketAddr::from_str(&format!("{}:{}", host, port))?;
    let service = make_service_fn(|_| async { Ok::<_, Error>(service_fn(any_handler)) } );
    let server = Server::bind(&addr).serve(service);
    println!("Listening on {}", addr);
    server.await?;
    Ok(())
}
