use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, ItemFn, LitStr};

pub fn internal_route(
    method: proc_macro2::TokenStream,
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    let name = &func.sig.ident;
    let vis = &func.vis;
    let path = parse_macro_input!(attr as LitStr);

    quote! {
        #vis struct #name {}

        impl salus::StaticRoute for #name {
            fn into(self) -> salus::Route {
                salus::Route {
                    path: #path.into(),
                    method: salus::http::Method::#method,
                    handler: Box::new(salus::_private::IntoNonGenericHandler::new(#name)),
                }
            }
        }

        #func
    }
    .into()
}
