use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

use crate::{
    handler::Handler,
    handler::{GenericHandler, IntoNonGenericHandler},
    Route,
};

pub struct Salus {
    routes: Vec<Route>,
}

impl Salus {
    pub fn new() -> Salus {
        Salus { routes: Vec::new() }
    }

    pub fn add_route<T: 'static>(
        &mut self,
        path: &str,
        method: http::Method,
        handler: impl GenericHandler<T> + 'static,
    ) {
        self.routes.push(Route {
            path: path.into(),
            method,
            handler: Box::new(IntoNonGenericHandler::new(handler)),
        });
    }

    pub fn routes(&self) -> &[Route] {
        &self.routes
    }

    async fn handle(&self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let path = req.uri().path();

        match self
            .routes
            .iter()
            .find(|route| route.path == path && route.method == req.method())
        {
            Some(route) => Ok(route
                .handler
                .handle(&mut req.into())
                .await
                .into_hyper_response()),
            None => Ok(Response::new(Body::from("Not found"))),
        }
    }

    pub async fn serve(self, addr: SocketAddr) {
        let salus = Arc::new(self);

        let make_svc = make_service_fn(move |_conn| {
            let salus = salus.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req: hyper::Request<Body>| {
                    let salus = salus.clone();
                    async move { salus.handle(req).await }
                }))
            }
        });

        let server = Server::bind(&addr).serve(make_svc);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

impl Default for Salus {
    fn default() -> Self {
        Self::new()
    }
}
