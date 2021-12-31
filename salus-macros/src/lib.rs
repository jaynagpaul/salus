use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::Ident;

mod route;

macro_rules! route_macro_builder {
    ($($l:ident, $u:ident); *) => {
        $(
            #[doc = "Creates the handler for the route with method "]
            #[doc = stringify!($u)]
            #[doc = "."]
            #[proc_macro_attribute]
            pub fn $l(attr: TokenStream, item: TokenStream) -> TokenStream {
                let method = quote!($u);
                route::internal_route(method, attr, item)
            }
        )*
    };
}

route_macro_builder!(
    get, GET;
    post, POST;
    put, PUT;
    delete, DELETE;
    patch, PATCH;
    head, HEAD;
    options, OPTIONS;
    connect, CONNECT;
    trace, TRACE
);
