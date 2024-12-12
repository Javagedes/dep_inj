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
    fn run(&mut self, config: &mut HashMap<TypeId, RefCell<Box<dyn Any>>>, services: &mut HashMap<TypeId, Box<dyn Any>>);
}

pub trait IntoComponent<Input> {
    type Component: Component;

    fn into_component(self) -> Self::Component;
}

#[derive(Default)]
pub struct ComponentManager {
    components: Vec<StoredComponent>,
    config: HashMap<TypeId, RefCell<Box<dyn Any>>>,
    services: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentManager {
    pub fn run(&mut self) {
        for system in self.components.iter_mut() {
            system.run(&mut self.config, &mut self.services);
        }
    }

    pub fn add_component<'a, I, C: Component + 'static>(
        &mut self,
        component: impl IntoComponent<I, Component = C>,
    ) {
        self.components.push(Box::new(component.into_component()));
    }

    pub fn add_config<C: Default + 'static>(&mut self, resource: C) {
        self.config
            .insert(TypeId::of::<C>(), RefCell::new(Box::new(resource)));
    }

    pub fn add_service<S: 'static>(&mut self, service: S) {
        self.services.insert(TypeId::of::<S>(), Box::new(service));
    }
}
