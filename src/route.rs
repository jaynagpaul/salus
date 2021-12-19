use crate::handler::{GenericHandler, Handler, IntoNonGenericHandler};
pub struct Route {
    pub path: String,
    pub method: http::Method,
    pub(crate) handler: Box<dyn Handler>,
}

impl Route {
    pub fn new<T: 'static>(
        path: String,
        method: http::Method,
        handler: impl GenericHandler<T> + 'static,
    ) -> Route {
        Route {
            path,
            method,
            handler: Box::new(IntoNonGenericHandler::new(handler)), /*Box::new(handler.into())*/
        }
    }
}
