use crate::Response;
use http::StatusCode;

pub trait Responder {
    fn response(self) -> Response;
}

impl Responder for Response {
    fn response(self) -> Response {
        self
    }
}

impl Responder for String {
    fn response(self) -> Response {
        Response::new(StatusCode::OK, self)
    }
}
