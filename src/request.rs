pub struct Request {
    inner: hyper::Request<hyper::Body>,
    body: Option<String>,
}

impl From<hyper::Request<hyper::Body>> for Request {
    fn from(inner: hyper::Request<hyper::Body>) -> Self {
        Self { inner, body: None }
    }
}

impl<'a> Request {
    pub async fn body(&'a mut self) -> &'a String {
        if self.body.is_none() {
            let b = hyper::body::to_bytes(self.inner.body_mut()).await.unwrap();
            let b = String::from_utf8(b.to_vec()).unwrap();

            self.body = Some(b);
        };

        self.body.as_ref().unwrap()
    }
}
