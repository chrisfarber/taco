use hyper::{Body, Request, Server};
use routerify::{Middleware, Router, RouterService};
use std::convert::Infallible;
use std::net::SocketAddr;

mod context;
mod handler;

// async fn main_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     let ctx = context::Context { state: 4 };
//     let handler1: Box<&dyn handler::Handler> = Box::new(&handler::hello);
//     let handler: Box<dyn handler::Handler> = match (req.method(), req.uri().path()) {
//         (&Method::GET, "/hello") => Box::new(handler::hello),
//         (_, _) => Box::new(handler::not_found),
//     };
//     return error_to_500(handler.invoke(&ctx, &req).await);
// }

// fn error_to_500<E>(result: Result<Response<Body>, E>) -> Result<Response<Body>, Infallible> {
//     match result {
//         Ok(resp) => Ok(resp),
//         Err(_) => Ok(Response::builder()
//             .status(500)
//             .body("oh no".into())
//             .unwrap()),
//     }
// }

// A middleware which logs an http request.
async fn logger(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    println!(
        "{} {} {}",
        // req.remote_addr(),
        "eh",
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

fn router() -> Router<Body, Infallible> {
    // Create a router and specify the logger middleware and the handlers.
    // Here, "Middleware::pre" means we're adding a pre middleware which will be executed
    // before any route handlers.
    Router::builder()
        // Specify the state data which will be available to every route handlers,
        // error handler and middlewares.
        .data(context::Context::new(42))
        .middleware(Middleware::pre(logger))
        .get("/hello", handler::hello)
        // .err_handler_with_info(error_handler)
        .build()
        .unwrap()
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

    let service = RouterService::new(router()).unwrap();
    let server = Server::bind(&addr).serve(service);
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
