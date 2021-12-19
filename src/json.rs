//! Json handling for Salus.

use http::{HeaderValue, StatusCode};

use crate::{argument::Argument, responder::InternalServerError, Request, Responder, Response};

/// Json `Argument` and `Responder` type
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
    fn respond(self, req: &Request) -> Response {
        let json = match serde_json::to_string(&self.0)
            .map_err(|e| InternalServerError(format!("Json failed to serialize: {}", e)))
        {
            Ok(json) => json,
            Err(e) => return e.respond(req),
        };

        let mut resp = Response::new(StatusCode::OK, json);
        resp.headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));

        resp
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
