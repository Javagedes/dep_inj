use core::any::TypeId;

use sdk::component::params::{Config, Service};

use crate::Storage;

pub trait ComponentParam {
    type Item<'new>;

    // Retrieves the component from storage
    fn retrieve<'r>(storage: &'r Storage) -> Self::Item<'r>;
    fn exists<'r>(storage: &'r Storage) -> bool;
}

impl<'res, T: Default + 'static> ComponentParam for Config<'res, T> {
    type Item<'new> = Config<'new, T>;

    fn retrieve<'r>(storage: &'r Storage) -> Self::Item<'r> {
        Config::from(storage.config.get(&TypeId::of::<T>()).unwrap())
    }

    // Config will always exist, as it is created with a default value when registering.
    fn exists<'r>(_: &'r Storage) -> bool {
        true
    }
}

impl<'res, T: 'static> ComponentParam for Service<'res, T> {
    type Item<'new> = Service<'new, T>;

    fn retrieve<'r>(storage: &'r Storage) -> Self::Item<'r> {
        Service::from(storage.services.get(&TypeId::of::<T>()).unwrap())
    }

    fn exists<'r>(storage: &'r Storage) -> bool {
        storage.services.contains_key(&TypeId::of::<T>())
    }
}

macro_rules! impl_component_param_tuple {
    ($($param: ident), *) => {
        impl<$($param: ComponentParam),*> ComponentParam for ($($param,)*) {
            type Item<'new> = ($($param::Item<'new>,)*);

            fn retrieve<'r>(_storage: &'r Storage) -> Self::Item<'r> {
                ($($param::retrieve(_storage),)*)
            }

            #[allow(unused_mut)]
            fn exists<'r>(_storage: &'r Storage) -> bool {
                let mut exists = true;
                $(
                    exists &= $param::exists(_storage);
                )*
                exists
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