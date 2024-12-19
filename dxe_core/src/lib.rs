#![no_std]

mod access;
mod function_component;
mod params;
mod struct_component;
mod unsafe_storage;

extern crate alloc;

use access::Access;
use alloc::{borrow::Cow, boxed::Box, vec::Vec};
use sdk::component::Storage;
use unsafe_storage::UnsafeStorageCell;

type StoredComponent = Box<dyn Component>;

#[derive(Default)]
struct MetaData {
    /// The read / write parameter access requirements for the component.
    access: Access,
    /// The name of the component.
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
    /// Runs the component when it does not have exclusive access to the storage.
    ///
    /// # Safety
    ///
    /// - Each Parameter must properly register its access, so the scheduler can
    ///   ensure that there are no data conflicts.
    unsafe fn run_unsafe(&mut self, storage: UnsafeStorageCell) -> bool;

    /// Runs the component with exclusive access to the storage.
    ///
    /// Due to this, any deferred storage updates can also be performed.
    fn run(&mut self, storage: &mut Storage) -> bool {
        let storage_cell = UnsafeStorageCell::from(storage);
        let result = unsafe { self.run_unsafe(storage_cell) };
        // storage.apply_deferred()
        result
    }
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
        loop {
            let len = self.components.len();
            self.components
                .retain_mut(|component| !component.run(&mut self.storage));
            if len == self.components.len() {
                break;
            }
        }
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
