extern crate alloc;

use core::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
};

use alloc::{boxed::Box, vec::Vec};
use hashbrown::HashMap;
use r_efi::efi::Guid;

use crate::protocol::Protocol;

pub struct SparseVec<V> {
    values: Vec<Option<V>>,
}

impl<V> SparseVec<V> {
    /// Creates a new empty [SparseVec].
    pub const fn new() -> Self {
        Self { values: Vec::new() }
    }

    #[inline]
    /// Returns true if the [SparseVec] contains a value at the given index.
    pub fn contains(&self, index: usize) -> bool {
        self.values.get(index).map(|v| v.is_some()).unwrap_or(false)
    }

    #[inline]
    /// Returns the value at the given index, if it exists.
    pub fn get(&self, index: usize) -> Option<&V> {
        self.values.get(index).map(|v| v.as_ref()).unwrap_or(None)
    }

    #[inline]
    /// Inserts a value at the given index.
    pub fn insert(&mut self, index: usize, value: V) {
        if index >= self.values.len() {
            self.values.resize_with(index + 1, || None);
        }
        self.values[index] = Some(value);
    }
}

// TODO: Flesh out this struct. Probably need something custom, not just a hashmap. Probably
// just an array storage where the stored item maintains a reference to its original type.
pub struct Storage {
    configs: SparseVec<RefCell<Box<dyn Any>>>,
    config_indices: HashMap<TypeId, usize>,
    protocol_db: HashMap<Guid, Box<dyn Any>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            configs: SparseVec::new(),
            config_indices: HashMap::new(),
            protocol_db: HashMap::new(),
        }
    }

    #[inline]
    pub fn register_config<C: Default + 'static>(&mut self) -> usize {
        self.get_or_register_resource(TypeId::of::<C>())
    }

    pub fn get_or_register_resource(&mut self, id: TypeId) -> usize {
        let idx = self.config_indices.len();
        *self.config_indices.entry(id).or_insert(idx)
    }

    /// Adds a config to the storage if one does not already exist.
    #[inline]
    pub fn try_add_config<C: Default + 'static>(&mut self, id: usize, config: C) {
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

    pub fn contains_protocol(&self, guid: &Guid) -> bool {
        self.protocol_db.contains_key(guid)
    }

    pub fn add_protocol<P: Protocol + 'static>(&mut self, protocol: P) {
        self.protocol_db.insert(*P::guid(), Box::new(protocol));
    }

    pub fn get_protocol_untyped(&self, guid: &Guid) -> &Box<dyn Any> {
        self.protocol_db.get(guid).expect("Protocol Exists")
    }
}
