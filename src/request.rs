use crate::Salus;

/// Represents an HTTP request.
pub struct Request<'s> {
    inner: hyper::Request<hyper::Body>,
    body: Option<String>,
    /// The salus instance that received the request.
    pub salus: &'s Salus,
}

impl<'s> Request<'s> {
    /// Returns the request's body as a string, caching it for future calls.
    pub async fn body(&mut self) -> &String {
        if self.body.is_none() {
            let b = hyper::body::to_bytes(self.inner.body_mut()).await.unwrap();
            let b = String::from_utf8(b.to_vec()).unwrap();

            self.body = Some(b);
        };

        self.body.as_ref().unwrap()
    }

    /// Creates a new request from a `hyper::Request` and attaches it to the given `Salus` instance.
    pub fn new(inner: hyper::Request<hyper::Body>, salus: &'s Salus) -> Self {
        Self {
            inner,
            body: None,
            salus,
        }
    }
}
