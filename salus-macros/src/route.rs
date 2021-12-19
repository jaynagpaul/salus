// use proc_macro::TokenStream;
// use quote::{quote, ToTokens};
// use syn::{parse_macro_input, ItemFn};

// pub fn internal_route(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let func = parse_macro_input!(item as ItemFn);

//     let name = func.sig.ident.to_string();
//     let attr_string = attr.to_string();
//     let (method, path) = attr_string.split_once(" ").unwrap();
//     let route_impl = quote! {
//         struct #name;

//         impl salus::StaticRoute for #name {
//             fn method() -> salus::http::Method {
//                 salus::http::Method::#method
//             }

//             fn path() -> String {
//                 #path
//             }
//         }
//     };

//     route_impl.into()
// }
