use std::marker::PhantomData;

pub use salus_macros::ComponentGraph;

/// A component that can be registered in the component graph.
pub trait Component<'a> {
    type Output;
}

pub trait InputMarker<'a> {
    type Input;

    fn on_update(&mut self, _update: Self::Input) {}
}

/// A marker type used for inputs.
pub struct Input<T>(PhantomData<T>);

impl<T> Input<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for Input<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> InputMarker<'a> for Input<T> {
    type Input = T;
}

pub struct StoredInput<T>(pub T);

impl<T> StoredInput<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T: Default> Default for StoredInput<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<'a, T> InputMarker<'a> for StoredInput<T> {
    type Input = T;

    fn on_update(&mut self, update: Self::Input) {
        self.0 = update;
    }
}
