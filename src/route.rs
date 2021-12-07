use crate::Handler;
pub struct Route {
    pub path: String,
    pub method: http::Method,
    pub(crate) handler: Box<dyn Handler + Send + Sync>,
}

impl Route {
    pub fn new<F>(path: String, method: http::Method, handler: impl Handler) -> Route {
        Route {
            path,
            method,
            handler: Box::new(handler),
        }
    }
}
