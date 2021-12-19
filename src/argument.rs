use crate::request::Request;

#[async_trait::async_trait]
pub trait Argument: Send + Sync {
    async fn from_request(request: &mut Request) -> Option<Self>
    where
        Self: std::marker::Sized;
}
