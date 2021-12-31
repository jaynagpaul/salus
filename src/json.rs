//! Json handling for Salus.

use http::{HeaderValue, StatusCode};

use crate::{argument::Argument, responder::InternalServerError, Request, Responder, Response};

/// Json `Argument` and `Responder` type
pub struct Json<T>(pub T);

impl<T> std::ops::Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::*;

    use super::*;

    #[tokio::test]
    async fn test_json_responder() {
        let mut salus = Salus::new();

        #[derive(Serialize)]
        struct Response {
            name: String,
        };

        async fn handler() -> Json<Response> {
            let name = "Jay".to_string();

            Response { name }.into()
        }

        salus.get("/", handler);

        let handle = tokio::spawn(salus.serve("localhost", 8080));

        let mut client = reqwest::Client::new();
        let res = client
            .get("http://localhost:8080/")
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body = res.text().await.unwrap();

        assert_eq!(body.trim(), r#"{"name":"Jay"}"#);

        handle.abort();
    }
}
