use std::{
    convert::Infallible,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request as HyperRequest, Response, Server,
};

use crate::{
    handler::{GenericHandler, IntoNonGenericHandler},
    state::StateMap,
    Request, Route, State,
};

/// The `Salus` struct is the main entry point of the library,
/// represents a server instance.
pub struct Salus {
    routes: Vec<Route>,
    pub(crate) state_map: StateMap,
}

macro_rules! route_builder {
    ($l:ident, $u:ident) => {
        #[doc = "Adds a route to the Salus server with method "]
        #[doc = stringify!($u)]
        #[doc = "."]
        pub fn $l<T: 'static>(&mut self, path: &str, handler: impl GenericHandler<T> + 'static) {
            self.add_route(path, http::Method::$u, handler);
        }
    };
}

impl Salus {
    #[must_use]
    /// Creates a new Salus instance.
    pub fn new() -> Salus {
        Salus {
            routes: Vec::new(),
            state_map: StateMap::new(),
        }
    }

    route_builder!(get, GET);
    route_builder!(post, POST);
    route_builder!(put, PUT);
    route_builder!(delete, DELETE);
    route_builder!(patch, PATCH);
    route_builder!(head, HEAD);
    route_builder!(options, OPTIONS);
    route_builder!(connect, CONNECT);
    route_builder!(trace, TRACE);

    /// Registers a route with the Salus server.
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

    #[must_use]
    /// Returns all registered routes.
    pub fn routes(&self) -> &[Route] {
        &self.routes
    }

    async fn handle(&self, req: HyperRequest<Body>) -> Result<Response<Body>, Infallible> {
        let path = req.uri().path();

        match self
            .routes
            .iter()
            .find(|route| route.path == path && route.method == req.method())
        {
            Some(route) => Ok(route
                .handler
                .handle(&mut Request::new(req, self))
                .await
                .into_hyper_response()),
            None => Ok(Response::new(Body::from("Not found"))),
        }
    }

    /// Starts the server on the given address.
    ///
    /// Take a look at `Salus::try_serve` for a non-panicking version.
    pub async fn serve(self, host: &str, port: u16) {
        self.try_serve(host, port).await.expect("Failed to serve");
    }

    /// Same as `Salus::serve`, but returns an error instead of panicking.
    pub async fn try_serve(self, host: &str, port: u16) -> Result<(), Error> {
        let salus = Arc::new(self);

        let make_svc = make_service_fn(|_conn| {
            let salus = salus.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req: hyper::Request<Body>| {
                    let salus = salus.clone();
                    async move { salus.handle(req).await }
                }))
            }
        });

        let host = if host == "localhost" {
            IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
        } else {
            host.parse()?
        };
        let addr = SocketAddr::from((host, port));

        let server = Server::try_bind(&addr)?.serve(make_svc);

        salus.display_startup_info(&addr);
        server.await?;

        Ok(())
    }

    fn display_startup_info(&self, addr: &SocketAddr) {
        println!("Salus is running on {}", addr);

        for route in self.routes() {
            println!("{}: {}", route.method, route.path);
        }

        println!();
        println!("To stop the server, press Ctrl+C");
    }

    /// Manage the state
    pub fn manage<T>(&mut self, state: T)
    where
        T: Send + Sync + 'static,
    {
        self.state_map.insert(State::new(state));
    }
}

/// Possible errors that can occure from `Salus::serve`.
#[derive(Debug)]
pub enum Error {
    /// Invalid host was given.
    InvalidHost(std::net::AddrParseError),

    /// Failed to bind to the given address / start the server.
    ServerError(hyper::Error),
}

impl From<std::net::AddrParseError> for Error {
    fn from(e: std::net::AddrParseError) -> Self {
        Self::InvalidHost(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Self::ServerError(e)
    }
}

impl Default for Salus {
    fn default() -> Self {
        Self::new()
    }
}
