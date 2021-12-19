use std::{future::Future, marker::PhantomData, pin::Pin};

use crate::{argument::Argument, Request, Responder, Response};

use async_trait::async_trait;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, req: &mut Request) -> Response;
}

/// Wrapper type to convert a `GenericHandler` into a `Handler`.
pub(crate) struct IntoNonGenericHandler<H, T> {
    handler: H,
    _marker: PhantomData<fn() -> T>,
}

impl<H, T> IntoNonGenericHandler<H, T> {
    pub(crate) fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<H, T> Handler for IntoNonGenericHandler<H, T>
where
    H: GenericHandler<T>,
{
    async fn handle(&self, req: &mut Request) -> Response {
        self.handler.handle(req).await
    }
}

/// Used for implementing Handler for various function types
#[async_trait]
pub trait GenericHandler<T>: Send + Sync + 'static {
    async fn handle(&self, req: &mut Request) -> Response;
}

#[async_trait]
impl<F: 'static, Fut, R: Responder, A> GenericHandler<(A,)> for F
where
    F: Send + Sync + Fn(A) -> Fut,
    Fut: Future<Output = R> + Send,
    A: Argument,
{
    async fn handle(&self, req: &mut Request) -> Response {
        let arg1 = match A::from_request(req).await {
            Some(a) => a,
            None => todo!("Missing argument"),
        };

        Responder::response(self(arg1).await)
    }
}

/// A handler impl for a synchronous function with no input.
#[async_trait]
impl<F: 'static, Fut, R: Responder> GenericHandler<()> for F
where
    F: Send + Sync + Fn() -> Fut,
    Fut: Future<Output = R> + Send,
{
    async fn handle(&self, _: &mut Request) -> Response {
        Responder::response(self().await)
    }
}
