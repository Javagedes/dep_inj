use core::{cell::UnsafeCell, marker::PhantomData, ptr};

use sdk::component::Storage;

/// A wrapper around a reference to a [Storage] object that allows for unsafe mutable
/// access to the storage.
///
/// ## Safety
///
/// The caller must ensure that multiple mutable accesses to the same object in storage
/// do not occur at the same time and that no other references to the storage exist at
/// the time of a structural change to storage (like inserting or deleting an object).
#[derive(Copy, Clone)]
pub struct UnsafeStorageCell<'s>(
    *mut Storage,
    PhantomData<(&'s Storage, &'s UnsafeCell<Storage>)>,
);

unsafe impl Send for UnsafeStorageCell<'_> {}
unsafe impl Sync for UnsafeStorageCell<'_> {}

impl<'s> From<&'s mut Storage> for UnsafeStorageCell<'s> {
    fn from(storage: &'s mut Storage) -> Self {
        UnsafeStorageCell::new_mutable(storage)
    }
}

impl<'s> From<&'s Storage> for UnsafeStorageCell<'s> {
    fn from(storage: &'s Storage) -> Self {
        UnsafeStorageCell::new_readonly(storage)
    }
}

impl<'s> UnsafeStorageCell<'s> {
    pub fn new_readonly(storage: &'s Storage) -> Self {
        Self(ptr::from_ref(storage).cast_mut(), PhantomData)
    }

    pub fn new_mutable(storage: &'s mut Storage) -> Self {
        Self(ptr::from_mut(storage), PhantomData)
    }

    pub fn storage_mut(self) -> &'s mut Storage {
        unsafe { &mut *self.0 }
    }

    pub fn storage(self) -> &'s Storage {
        unsafe { &*self.0 }
    }
}
