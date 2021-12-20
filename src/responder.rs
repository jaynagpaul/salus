use crate::{Request, Response};
use http::StatusCode;

/// Any type that can be converted into a that can be used as a handler return type and creates a
/// `Response`.
pub trait Responder {
    /// Create the response.
    fn respond(self, req: &Request) -> Response;
}

pub struct InternalServerError(pub String);

impl Responder for InternalServerError {
    fn respond(self, req: &Request) -> Response {
        req.salus.warn(&self.0);

        Response::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    }
}

impl Responder for Response {
    fn respond(self, _: &Request) -> Response {
        self
    }
}

impl<T, E> Responder for Result<T, E>
where
    T: Responder,
    E: Responder,
{
    fn respond(self, req: &Request) -> Response {
        match self {
            Ok(t) => t.respond(req),
            Err(e) => e.respond(req),
        }
    }
}

impl<T: Responder> Responder for Option<T> {
    // TODO: implement 404 catcher
    fn respond(self, req: &Request) -> Response {
        match self {
            Some(t) => t.respond(req),
            None => Response::new(StatusCode::NOT_FOUND, "Optional Not Found"),
        }
    }
}

impl Responder for String {
    fn respond(self, _: &Request) -> Response {
        Response::new(StatusCode::OK, self)
    }
}
