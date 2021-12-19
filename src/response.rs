pub struct Response {
    inner: hyper::Response<hyper::Body>,
}

impl std::ops::Deref for Response {
    type Target = hyper::Response<hyper::Body>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<hyper::Response<hyper::Body>> for Response {
    fn from(inner: hyper::Response<hyper::Body>) -> Self {
        Self { inner }
    }
}

impl Response {
    pub fn new(status: http::StatusCode, body: impl Into<hyper::Body>) -> Self {
        Response {
            inner: hyper::Response::builder()
                .status(status)
                .body(body.into())
                .unwrap(),
        }
    }

    pub fn builder() -> http::response::Builder {
        hyper::Response::builder()
    }

    pub(crate) fn into_hyper_response(self) -> hyper::Response<hyper::Body> {
        self.inner
    }
}
