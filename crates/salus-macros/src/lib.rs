mod component_graph;

use component_graph::ComponentGraph;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Attribute, Data, DeriveInput, Fields,
    Ident, Type,
};

#[proc_macro_derive(ComponentGraph, attributes(depends_on))]
pub fn derive_component_graph(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("ComponentGraph only supports structs with named fields"),
        },
        _ => panic!("ComponentGraph can only be derived for structs"),
    };

    let mut graph = ComponentGraph::new(name.clone());
    graph.from_fields(fields);

    graph_to_tokens(graph)
}

fn graph_to_tokens(graph: ComponentGraph) -> TokenStream {
    let mut tokens = TokenStream::new();
    let graph_name = &graph.name;

    for (component_name, component) in graph.components.iter() {
        let ty = &component.ty;

        let propagate_update_name = Ident::new(
            format!("__salus_propagate_{}_update", component_name).as_str(),
            component_name.span(),
        );

        let new_tokens: TokenStream = if component.is_input() {
            let update_field_name = Ident::new(
                format!("update_{}", component_name).as_str(),
                component_name.span(),
            );

            let subscribers = component.subscribers.iter().map(|subscriber| {
                Ident::new(
                    format!("self.{}", subscriber).as_str(),
                    subscriber.span(),
                )
            }).map(|subscriber| {
                let one_update_fn = Ident::new(
                    format!("on_{}_update", component_name).as_str(),
                    subscriber.span(),
                );

               quote!{
                #subscriber.#one_update_fn(update);
               } 
            });

            quote! {
                impl #graph_name {
                    pub fn #update_field_name(&mut self, update: #ty) {
                        self.#component_name = update;
                        self.#propagate_update_name(update);
                    }

                    // #[doc(hidden)]
                    fn #propagate_update_name(&mut self, update: #ty) {
                        // TODO: propagate update to subscribers
                        todo!()
                    }
                }
            }
        } else {
            quote! {
                impl #graph_name {
                    // #[doc(hidden)]
                    fn #propagate_update_name(&mut self, update: <#ty as salus::Component>::Output) {
                        todo!()
                    }
                }
            }
        }.into();

        tokens.extend(new_tokens);
    }

    tokens
}

// For each input field:
//  - add a pub update_field() mathod

// For each component field:
//  - Define a Trait OnFieldUpdate
//  - Define a notify_subscribers()
