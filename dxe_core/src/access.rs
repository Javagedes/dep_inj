extern crate alloc;

use core::fmt;
use fixedbitset::FixedBitSet;

/// Access requirements for a component.
#[derive(Default)]
pub struct Access {
    config_writes: FixedBitSet,
    config_read_and_writes: FixedBitSet,
    exclusive: bool,
}

impl Access {
    /// Registers a write access to a config resource.
    pub fn add_config_write(&mut self, id: usize) {
        self.config_writes.grow_and_insert(id);
        self.config_read_and_writes.grow_and_insert(id);
    }

    /// Registers a read access to a config resource.
    pub fn add_config_read(&mut self, id: usize) {
        self.config_read_and_writes.grow_and_insert(id);
    }

    /// Returns true if the component needs mutable access to the config resources denoted by `id`.
    pub fn has_config_write(&self, id: usize) -> bool {
        self.config_writes.contains(id)
    }

    /// Returns true if the component needs access read to the config resources denoted by `id`.
    pub fn has_config_read(&self, id: usize) -> bool {
        self.config_read_and_writes.contains(id)
    }
}

impl fmt::Debug for Access {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Access")
            .field("config_writes", &PrettyFixedBitSet(&self.config_writes))
            .field("exclusive", &self.exclusive)
            .finish()
    }
}

pub struct PrettyFixedBitSet<'a>(&'a FixedBitSet);

impl fmt::Debug for PrettyFixedBitSet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.0.ones().map(|i| i as u32))
            .finish()
    }
}
