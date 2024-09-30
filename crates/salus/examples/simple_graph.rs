use std::marker::PhantomData;

use salus::{Component, ComponentGraph, Input};

// #[derive(ComponentGraph)]
pub struct SimpleGraph<'a> {
    input: i32,

    // #[depends_on(input)]
    component: MyComponent<'a>,
}

// #[derive(ComponentGraph)]
// pub struct MoreComplexGraph {
//     ephemeral_input: Input<i32>, // 0-sized
//     last_datapacket: InputContainer<DataPacket>,

//     #[depends_on(ephemeral_input, last_datapacket)]
//     component: MyComponent,
// }

struct Logger;

struct System<'a> {
    logger: Logger,
    graph: SimpleGraph<'a>,
}

// This generates the following public interface:
//
// impl SimpleGraph {
//     pub fn update_input(&mut self, input: i32);
// }
// trait OnInputUpdate<'a>: Component<'a> {
//     fn on_input_update(&'a mut self, input: i32);
// }
//
// trait OnComponentUpdate<'a>: Component<'a> {
//     fn on_component_update(&'a mut self, input: i32);
// }
//
// The depends_on attribute ensures that the component is updated when the input is updated.

struct MyComponent<'a> {
    logger: &'a Logger,
}

impl<'a> Component<'a> for MyComponent<'a> {
    type Output = bool;
}

fn main() {
    let logger = Logger;

    let system = System {
        logger: Logger,

        graph: SimpleGraph {
            input: 0,
            component: MyComponent { logger: &logger },
        },
    };

    // let mut graph = SimpleGraph {
    //     input: 0,
    //     component: MyComponent { logger: &logger },
    // };

    // graph.update_input(10);
}
