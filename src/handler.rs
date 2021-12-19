//! Types and traits related to the request handler.

use std::{future::Future, marker::PhantomData};

use crate::{argument::Argument, Request, Responder, Response};

use async_trait::async_trait;

/// A handler that can be used to respond to requests.
/// Used internally to dynamic dispatch between different handler types.
/// Look at `GenericHandler` for the type used in routing to accept function types.
#[async_trait]
pub trait Handler: Send + Sync {
    /// Handle a request, returning a response.
    async fn handle(&self, req: &mut Request) -> Response;
}

/// Wrapper type to convert a `GenericHandler` into a `Handler`.
/// _marker is used to "utilize" the T generic parameter, a workaround for not being able to
/// use a generic parameter solely to restrict the type of the `GenericHandler` trait.
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
/// Kept generic to allow multiple implementations for functions with different signatures
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

        Responder::respond(self(arg1).await, req)
    }
}

/// A handler impl for a synchronous function with no input.
#[async_trait]
impl<F: 'static, Fut, R: Responder> GenericHandler<()> for F
where
    F: Send + Sync + Fn() -> Fut,
    Fut: Future<Output = R> + Send,
{
    async fn handle(&self, req: &mut Request) -> Response {
        Responder::respond(self().await, req)
    }
}
