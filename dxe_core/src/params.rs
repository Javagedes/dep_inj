use r_efi::efi::Guid;
use sdk::{
    component::{
        params::{Config, ConfigMut, Protocol},
        Storage,
    },
    protocol,
};

use crate::{unsafe_storage::UnsafeStorageCell, MetaData};

/// Allows automatic retrieval of an implementing type from storage for dependency injection.
pub trait ComponentParam {
    /// Persistent state for the parameter.
    type State: Send + Sync + 'static;
    /// The item type that is retrieved from storage.
    type Item<'w, 'state>;

    /// Retrieves the parameter from storage.
    ///
    /// ## Safety
    ///
    /// - The parameter storage access must be properly registered with the caller.
    unsafe fn retrieve<'w, 'state>(
        _state: &'state mut Self::State,
        _storage: UnsafeStorageCell<'w>,
    ) -> Self::Item<'w, 'state>;

    /// Validates that the parameter exists, and is in a state that can be retrieved from storage.
    fn validate<'r>(_state: &Self::State, _storage: UnsafeStorageCell) -> bool;

    /// Initializes the parameter, if necessary.
    fn initialize(storage: &mut Storage, meta: &mut MetaData) -> Self::State;
}

impl ComponentParam for &mut Storage {
    type State = ();
    type Item<'w, 'state> = &'w mut Storage;

    unsafe fn retrieve<'w, 'state>(
        _state: &'state mut Self::State,
        storage: UnsafeStorageCell<'w>,
    ) -> Self::Item<'w, 'state> {
        storage.storage_mut()
    }

    fn validate<'r>(_state: &Self::State, _storage: UnsafeStorageCell) -> bool {
        true
    }

    fn initialize(_storage: &mut Storage, meta: &mut MetaData) {
        meta.access.set_exclusive();
    }
}

impl ComponentParam for &Storage {
    type State = ();
    type Item<'w, 'state> = &'w Storage;

    unsafe fn retrieve<'w, 'state>(
        _state: &'state mut Self::State,
        storage: UnsafeStorageCell<'w>,
    ) -> Self::Item<'w, 'state> {
        storage.storage()
    }

    fn validate<'r>(_state: &Self::State, _storage: UnsafeStorageCell) -> bool {
        true
    }

    fn initialize(_storage: &mut Storage, meta: &mut MetaData) {
        meta.access.set_exclusive();
    }
}

impl<'c, T: Default + 'static> ComponentParam for Config<'c, T> {
    // For this implementation of ComponentParam, `State` is used to store the global id of the Config object.
    // This prevents the need to look it up every time we attempt to retrieve the Config object from storage
    // for a system. This improves performance when we have systems that fail to run over many attempts
    // while waiting for some required resource to be registered.
    type State = usize;
    type Item<'w, 'state> = Config<'w, T>;

    unsafe fn retrieve<'w, 'state>(
        state: &'state mut Self::State,
        storage: UnsafeStorageCell<'w>,
    ) -> Self::Item<'w, 'state> {
        let id = *state;
        Config::from(storage.storage().get_config_untyped(id))
    }

    // Config will always exist, because a default value is registered during `initialize` if it does not already
    // exist.
    fn validate<'r>(_state: &Self::State, _storage: UnsafeStorageCell) -> bool {
        true
    }

    // Note: For this implementation, we get the global id of the config object and store it in the param state so that
    // if we need to attempt to retrieve the config object from storage many times (This happens when a component
    // fails to run because it is waiting for some other ComponentParam to be available)), it can be done quickly.
    //
    // Since The config object can be mutable, we register the access type here and check for conflicts with other
    fn initialize(storage: &mut Storage, meta: &mut MetaData) -> Self::State {
        let id = storage.register_config::<T>();
        storage.try_add_config(id, T::default());

        assert!(
            !meta.access.has_config_write(id),
            "Config<{}> in system {} conflicts with a previous ConfigMut<{0}> access.",
            core::any::type_name::<T>(),
            meta.name,
        );

        meta.access.add_config_read(id);
        id
    }
}

// An example of mutating Component parameters, but probably won't keep this as config should probably
// remain immutable.
impl<'c, T: Default + 'static> ComponentParam for ConfigMut<'c, T> {
    type State = usize;
    type Item<'w, 'state> = ConfigMut<'w, T>;

    unsafe fn retrieve<'w, 'state>(
        state: &'state mut Self::State,
        storage: UnsafeStorageCell<'w>,
    ) -> Self::Item<'w, 'state> {
        let id = *state;
        ConfigMut::from(storage.storage_mut().get_config_mut_untyped(id))
    }

    // Config will always exist, as it is created with a default value when registering.
    fn validate<'r>(_state: &Self::State, _storage: UnsafeStorageCell) -> bool {
        true
    }

    fn initialize(storage: &mut Storage, meta: &mut MetaData) -> Self::State {
        let id = storage.register_config::<T>();

        assert!(
            !meta.access.has_config_write(id),
            "ConfigMut<{}> in system {} conflicts with a previous ConfigMut<{0}> access.",
            core::any::type_name::<T>(),
            meta.name,
        );

        assert!(
            !meta.access.has_config_read(id),
            "ConfigMut<{}> in system {} conflicts with a previous Config<{0}> access.",
            core::any::type_name::<T>(),
            meta.name,
        );

        meta.access.add_config_write(id);
        id
    }
}

impl<'p, P: protocol::Protocol + 'static> ComponentParam for Protocol<'p, P> {
    type State = Guid;
    type Item<'w, 'state> = Protocol<'w, P>;

    unsafe fn retrieve<'w, 'state>(
        state: &'state mut Self::State,
        storage: UnsafeStorageCell<'w>,
    ) -> Self::Item<'w, 'state> {
        Protocol::from(storage.storage().get_protocol_untyped(state))
    }

    fn validate<'r>(state: &Self::State, storage: UnsafeStorageCell) -> bool {
        storage.storage().contains_protocol(state)
    }

    fn initialize(_storage: &mut Storage, _meta: &mut MetaData) -> Self::State {
        *P::guid()
    }
}

macro_rules! impl_component_param_tuple {
    ($($param: ident), *) => {
        #[allow(non_snake_case)]
        impl<$($param: ComponentParam),*> ComponentParam for ($($param,)*) {
            type State = ($($param::State,)*);
            type Item<'w, 'state> = ($($param::Item::<'w, 'state>,)*);

            unsafe fn retrieve<'w, 'state>(state: &'state mut Self::State, _storage: UnsafeStorageCell<'w>) -> Self::Item<'w, 'state> {
                let ($($param,)*) = state;
                ($($param::retrieve($param, _storage),)*)
            }

            #[allow(unused_mut)]
            fn validate<'r>(state: &Self::State, _storage: UnsafeStorageCell) -> bool {
                let ($($param,)*) = state;
                $($param::validate($param, _storage)&&)* true
            }

            fn initialize(_storage: &mut Storage, _meta: &mut MetaData) -> Self::State {
                (($($param::initialize(_storage, _meta),)*))
            }
        }
    }
}

impl_component_param_tuple!();
impl_component_param_tuple!(T1);
impl_component_param_tuple!(T1, T2);
impl_component_param_tuple!(T1, T2, T3);
impl_component_param_tuple!(T1, T2, T3, T4);
impl_component_param_tuple!(T1, T2, T3, T4, T5);
