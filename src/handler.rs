use std::future::Future;

use super::context::Context;
use async_trait::async_trait;

use hyper::{http::Error, Body, Request, Response};

// async fn answer(
//     context: &Context,
//     req: &Request<Body>,
// ) -> Result<Response<Body>, hyper::http::Error> {
//     Response::builder().status(200).body("hello\n".into())
// }

pub async fn hello(ctx: &Context, req: &Req) -> RespResult {
    Response::builder().status(200).body("hello\n".into())
}

pub async fn not_found(ctx: &Context, req: &Req) -> RespResult {
    Response::builder().status(404).body("not found.".into())
}

pub type Req = Request<Body>;
pub type Resp = Response<Body>;
pub type RespResult = Result<Resp, Error>;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn invoke(&self, context: &Context, req: &Req) -> RespResult;
}

#[async_trait]
impl<F: Send + Sync + 'static, Fut> Handler for F
where
    F: Fn(&Context, &Req) -> Fut,
    Fut: Future + Send + 'static,
    Fut::Output: RespResultTrait,
{
    async fn invoke(&self, ctx: &Context, req: &Req) -> RespResult {
        (self)(ctx, req).await.into_response()
    }
}

pub trait RespResultTrait: Send + Sized {
    fn into_response(self) -> RespResult;
}

impl RespResultTrait for RespResult {
    fn into_response(self) -> RespResult {
        self
    }
}
