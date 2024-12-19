#![no_std]

mod access;
mod function_component;
mod params;
mod storage;
mod struct_component;

extern crate alloc;
use core::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
};

use access::Access;
use alloc::{borrow::Cow, boxed::Box, vec::Vec};
use hashbrown::HashMap;
use storage::SparseVec;

type StoredComponent = Box<dyn Component>;

#[derive(Default)]
struct MetaData {
    access: Access,
    name: Cow<'static, str>,
}

impl MetaData {
    fn new<S>() -> Self {
        let name = core::any::type_name::<S>();
        Self {
            access: Access::default(),
            name: name.into(),
        }
    }
}

/// Allows an object to be executed by the ComponentManager.
trait Component {
    fn run(&mut self, storage: &mut Storage) -> bool;
    /// One time initialization of the component. Should set access requirements.
    fn initialize(&mut self, storage: &mut Storage);
    /// Returns the metadata of the component.
    fn metadata(&self) -> &MetaData;
}

/// Helper trait to convert an object into a Component.
trait IntoComponent<Input> {
    type Component: Component;

    fn into_component(self) -> Self::Component;
}

/// A manager for components.
pub struct ComponentManager {
    components: Vec<StoredComponent>,
    storage: Storage,
}

impl ComponentManager {
    /// Creates a new ComponentManager.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            storage: Storage::new(),
        }
    }

    /// Returns the number of components in the manager.
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// Runs all components in the manager.
    pub fn run(&mut self) {
        self.components
            .retain_mut(|component| !component.run(&mut self.storage));
    }

    #[allow(private_bounds)]
    /// Adds a component to the manager.
    pub fn add_component<'a, I, C: Component + 'static>(
        &mut self,
        component: impl IntoComponent<I, Component = C>,
    ) {
        let mut component = component.into_component();
        component.initialize(&mut self.storage);
        self.components.push(Box::new(component));
    }

    /// Adds a Configuration value to the manager.
    pub fn add_config<C: Default + 'static>(&mut self, config: C) {
        self.storage.add_config(config);
    }
}

// TODO: Flesh out this struct. Probably need something custom, not just a hashmap. Probably
// just an array storage where the stored item maintains a reference to its original type.
pub struct Storage {
    configs: SparseVec<RefCell<Box<dyn Any>>>,
    config_indices: HashMap<TypeId, usize>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            configs: SparseVec::new(),
            config_indices: HashMap::new(),
        }
    }

    #[inline]
    fn register_config<C: Default + 'static>(&mut self) -> usize {
        self.get_or_register_resource(TypeId::of::<C>())
    }

    fn get_or_register_resource(&mut self, id: TypeId) -> usize {
        let idx = self.config_indices.len();
        *self.config_indices.entry(id).or_insert(idx)
    }

    /// Adds a config to the storage if one does not already exist.
    #[inline]
    fn try_add_config<C: Default + 'static>(&mut self, id: usize, config: C) {
        if !self.configs.contains(id) {
            self.configs.insert(id, RefCell::new(Box::new(config)));
        }
    }

    #[inline]
    /// Adds a config to the storage, overwriting any existing config.
    pub fn add_config<C: Default + 'static>(&mut self, config: C) {
        let id = self.register_config::<C>();
        self.try_add_config(id, config);
    }

    /// Retrieves a config from the storage.
    pub fn get_config_untyped(&self, id: usize) -> Ref<Box<dyn Any>> {
        self.configs.get(id).expect("Config Exists").borrow()
    }

    /// Retrieves a mutable config from the storage.
    pub fn get_config_mut_untyped(&self, id: usize) -> RefMut<Box<dyn Any>> {
        self.configs.get(id).expect("Config Exists").borrow_mut()
    }
}
