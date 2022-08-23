use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn answer(req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
    Response::builder()
        .status(200)
        .header("woah", "dude\r\n")
        .body(req.into_body())
}

async fn echo_service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    error_to_500(answer(req).await)
}

fn error_to_500<E>(result: Result<Response<Body>, E>) -> Result<Response<Body>, Infallible> {
    match result {
        Ok(resp) => Ok(resp),
        Err(_) => Ok(Response::builder()
            .status(500)
            .body("oh no".into())
            .unwrap()),
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(echo_service)) });

    let server = Server::bind(&addr).serve(make_svc);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("listening.");
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
    println!("bye");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_addition_fails() {
        assert_eq!(1, 1);
    }
}
