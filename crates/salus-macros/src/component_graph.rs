use std::collections::HashMap;

use syn::{punctuated::Punctuated, token::Comma, Field, Fields, Ident, Type};

const DEPENDS_ON_ATTR: &str = "depends_on";

pub struct Component {
    pub ty: Type,

    pub dependencies: Vec<Ident>,
    pub subscribers: Vec<Ident>,
}

pub struct ComponentGraph {
    pub name: Ident,

    pub components: HashMap<Ident, Component>,
}

impl Component {
    pub fn new(ty: Type) -> Self {
        Self {
            ty,
            dependencies: vec![],
            subscribers: vec![],
        }
    }

    pub fn is_input(&self) -> bool {
        self.dependencies.is_empty()
    }

    pub fn add_dependency(&mut self, dependency: Ident) {
        self.dependencies.push(dependency);
    }

    pub fn add_subscriber(&mut self, subscriber: Ident) {
        self.subscribers.push(subscriber);
    }
}

impl ComponentGraph {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            components: HashMap::new(),
        }
    }

    pub fn add_subscriber(&mut self, component: &Ident, subscriber: &Ident) {
        self.components
            .get_mut(component)
            .unwrap()
            .add_subscriber(subscriber.clone());
    }

    pub fn add_dependency(&mut self, component: &Ident, dependency: &Ident) {
        self.components
            .get_mut(component)
            .unwrap()
            .add_dependency(dependency.clone());
    }

    pub fn from_fields(&mut self, fields: &Punctuated<Field, Comma>) {
        // Create a component for each field
        for field in fields {
            let ident = field.ident.clone();
            let ty = field.ty.clone();
            self.components
                .insert(ident.expect("Field must have an ident"), Component::new(ty));
        }

        // Add dependencies and subscribers to each component
        for field in fields {
            let depends_attr = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident(DEPENDS_ON_ATTR));

            let dependencies = match depends_attr {
                Some(attr) => extract_dependencies(attr),
                None => vec![],
            };

            for dependency in dependencies {
                self.add_dependency(&field.clone().ident.unwrap(), &dependency);
                self.add_subscriber(&dependency, &field.clone().ident.unwrap());
            }
        }

        self.validate();
    }

    fn validate(&self) {
        // todo!()
    }
}

fn extract_dependencies(attr: &syn::Attribute) -> Vec<Ident> {
    let parsed = attr
        .parse_args_with(Punctuated::<Ident, Comma>::parse_terminated)
        .unwrap();

    parsed.into_iter().collect()
}

// TODO: check for cycles
