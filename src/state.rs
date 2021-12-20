use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use crate::{argument::Argument, Request};

pub(crate) struct StateMap {
    map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl StateMap {
    // Create a new state map
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    // Insert a new value based on the TypeId of the value
    pub fn insert<T: Send + Sync + 'static>(&mut self, val: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(val));
    }

    // Get a value from the map with the given type.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }
}

#[derive(Clone)]
/// An `Argument` that can be used to extract managed state from the request.
pub struct State<T>(Arc<T>);

impl<T> State<T> {
    /// Create a new `State` from a `T`
    pub(crate) fn new(t: T) -> Self {
        Self(Arc::new(t))
    }
}

impl<T> std::ops::Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait::async_trait]
impl<T> Argument for State<T>
where
    T: Send + Sync + 'static,
{
    async fn from_request(req: &mut Request) -> Option<Self> {
        let state =
            req.salus.state_map.get::<State<T>>().unwrap_or_else(|| {
                panic!("State: {:?} not being managed by Salus", TypeId::of::<T>())
            });

        Some(State(state.0.clone()))
    }
}
