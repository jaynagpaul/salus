/// Represents an HTTP response.
pub struct Response {
    inner: hyper::Response<hyper::Body>,
}

impl std::ops::Deref for Response {
    type Target = hyper::Response<hyper::Body>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Response {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<hyper::Response<hyper::Body>> for Response {
    fn from(inner: hyper::Response<hyper::Body>) -> Self {
        Self { inner }
    }
}

impl Response {
    /// Creates a new response with the given status code and body.
    pub fn new(status: http::StatusCode, body: impl Into<hyper::Body>) -> Self {
        Response {
            inner: hyper::Response::builder()
                .status(status)
                .body(body.into())
                .unwrap(),
        }
    }

    #[must_use]
    /// Returns a `http::response::Builder` used to create a response.
    pub fn builder() -> http::response::Builder {
        hyper::Response::builder()
    }

    pub(crate) fn into_hyper_response(self) -> hyper::Response<hyper::Body> {
        self.inner
    }
}
