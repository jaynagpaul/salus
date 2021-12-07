use std::future::Future;

use crate::{Responder, Response};

use async_trait::async_trait;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn handle(&self) -> Response;
}

/// A handler impl for a synchronous function with no input.
#[async_trait]
impl<F, Fut, R: Responder> Handler for F
where
    F: Send + Sync + 'static + Fn() -> Fut,
    Fut: Future<Output = R> + Send + 'static,
{
    async fn handle(&self) -> Response {
        Responder::response(self().await)
    }
}
