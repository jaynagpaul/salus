pub struct Response {
    inner: hyper::Response<hyper::Body>,
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

    pub(crate) fn into_hyper_response(self) -> hyper::Response<hyper::Body> {
        self.inner
    }
}
