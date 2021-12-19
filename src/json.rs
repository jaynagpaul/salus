use crate::{argument::Argument, Responder, Response};

pub struct Json<T>(pub T);

impl<T> From<T> for Json<T>
where
    T: serde::Serialize,
{
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> Responder for Json<T>
where
    T: serde::Serialize,
{
    fn response(self) -> Response {
        Response::builder()
            .status(http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&self.0).unwrap().into())
            .unwrap()
            .into()
    }
}

#[async_trait::async_trait]
impl<T> Argument for Json<T>
where
    T: serde::de::DeserializeOwned + Send + Sync,
{
    async fn from_request(req: &mut crate::Request) -> Option<Self>
    where
        Self: std::marker::Sized,
    {
        let body = req.body().await;

        serde_json::from_str(body).map(Json).ok()
    }
}
