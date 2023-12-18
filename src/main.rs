use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;
use tokio::net::TcpListener as TokioTcpListener;

async fn handle_request(_: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("Hello, World!")))
}

#[tokio::main]
async fn main() {
    // Bind to 127.0.0.1:8080
    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();

    // Create a Tokio TcpListener
    let tokio_listener = TokioTcpListener::bind(&addr).await.unwrap();

    // Convert Tokio TcpListener to the standard library's TcpListener
    let std_listener = tokio_listener.into_std().unwrap();

    // Create the server by binding to the address
    let server = Server::from_tcp(std_listener).unwrap().serve(make_service_fn(|_conn| {
        async { Ok::<_, hyper::Error>(service_fn(handle_request)) }
    }));

    println!("Server running at http://{}", addr);

    // Run the server
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
