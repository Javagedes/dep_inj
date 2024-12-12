#![no_std]

pub mod params;
mod type_fn;
mod type_struct;

//pub use params::{Config, ConfigMut};

extern crate alloc;
use core::{
    any::{Any, TypeId},
    cell::RefCell,
};

use alloc::{boxed::Box, vec::Vec};
use hashbrown::HashMap;

type StoredComponent = Box<dyn Component>;

pub trait Component {
    fn run(&mut self, config: &mut HashMap<TypeId, RefCell<Box<dyn Any>>>);
}

pub trait IntoComponent<Input> {
    type Component: Component;

    fn into_component(self) -> Self::Component;
}

#[derive(Default)]
pub struct ComponentManager {
    components: Vec<StoredComponent>,
    config: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl ComponentManager {
    pub fn run(&mut self) {
        for system in self.components.iter_mut() {
            system.run(&mut self.config);
        }
    }

    pub fn add_component<'a, I, C: Component + 'static>(
        &mut self,
        component: impl IntoComponent<I, Component = C>,
    ) {
        self.components.push(Box::new(component.into_component()));
    }

    pub fn add_config<R: 'static>(&mut self, resource: R) {
        self.config
            .insert(TypeId::of::<R>(), RefCell::new(Box::new(resource)));
    }
}
