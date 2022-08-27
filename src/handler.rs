use std::convert::Infallible;

use super::context::Context;

use hyper::{Body, Error, Request, Response};

pub type Req = Request<Body>;
pub type Resp = Response<Body>;
pub type RespResult = Result<Resp, Infallible>;

pub async fn hello(req: Req) -> RespResult {
    Ok(Response::builder()
        .status(200)
        .body("hello\n".into())
        .unwrap())
}

// pub async fn not_found(ctx: &Context, req: &Req) -> RespResult {
//     Response::builder().status(404).body("not found.".into())
// }
