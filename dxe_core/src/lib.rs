#![no_std]

mod params;
mod access;
mod function_component;
mod struct_component;

extern crate alloc;
use core::{
    any::{Any, TypeId},
    cell::RefCell,
};

use alloc::{boxed::Box, vec::Vec};
use hashbrown::HashMap;

type StoredComponent = Box<dyn Component>;

pub trait Component {
    fn run(&mut self, storage: &mut Storage) -> bool;
}

pub trait IntoComponent<Input> {
    type Component: Component;

    fn into_component(self) -> Self::Component;
}

#[derive(Default)]
pub struct ComponentManager {
    pub components: Vec<StoredComponent>,
    storage: Storage,
}

impl ComponentManager {
    pub fn run(&mut self) {
        self.components.retain_mut(|component| !component.run(&mut self.storage));
    }

    pub fn add_component<'a, I, C: Component + 'static>(
        &mut self,
        component: impl IntoComponent<I, Component = C>,
    ) {
        self.components.push(Box::new(component.into_component()));
    }

    pub fn add_config<C: Default + 'static>(&mut self, resource: C) {
        self.storage.config
            .insert(TypeId::of::<C>(), RefCell::new(Box::new(resource)));
    }

    pub fn add_service<S: 'static>(&mut self, service: S) {
        self.storage.services.insert(TypeId::of::<S>(), Box::new(service));
    }
}

#[derive(Default)]
pub struct Storage {
    // Example: We use RefCell to allow for interior mutability by taking as Ref or RefMut
    config: HashMap<TypeId, RefCell<Box<dyn Any>>>,
    // Example: We don't use RefCell here because we don't need interior mutability
    services: HashMap<TypeId, Box<dyn Any>>,
}