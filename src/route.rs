use crate::handler::{GenericHandler, Handler, IntoNonGenericHandler};
/// How `Salus` stores the route information
pub struct Route {
    /// The path of the route
    pub path: String,

    /// HTTP method of the route
    pub method: http::Method,
    pub handler: Box<dyn Handler>,
}

impl Route {
    /// Creates a new route.
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
