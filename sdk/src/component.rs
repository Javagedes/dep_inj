pub mod params {
    extern crate alloc;

    use alloc::boxed::Box;
    use core::{
        any::Any,
        cell::{Ref, RefMut},
        marker::PhantomData,
        ops::{Deref, DerefMut},
    };

    pub struct Config<'res, T: Default + 'static> {
        value: Ref<'res, Box<dyn Any>>,
        _marker: PhantomData<T>,
    }

    impl<'res, T: Default + 'static> Deref for Config<'_, T> {
        type Target = T;

        fn deref(&self) -> &T {
            self.value.downcast_ref().unwrap()
        }
    }

    impl<'res, T: Default + 'static> From<Ref<'res, Box<dyn Any>>> for Config<'res, T> {
        fn from(value: Ref<'res, Box<dyn Any>>) -> Self {
            Config {
                value,
                _marker: PhantomData,
            }
        }
    }

    // An example of mutating Component parameters, but probably won't keep this exact implementation
    // as config should probably remain immutable.
    pub struct ConfigMut<'res, T: Default + 'static> {
        value: RefMut<'res, Box<dyn Any>>,
        _marker: PhantomData<T>,
    }

    impl<'res, T: Default + 'static> Deref for ConfigMut<'_, T> {
        type Target = T;

        fn deref(&self) -> &T {
            self.value.downcast_ref().unwrap()
        }
    }

    impl<'res, T: Default + 'static> DerefMut for ConfigMut<'_, T> {
        fn deref_mut(&mut self) -> &mut T {
            self.value.downcast_mut().unwrap()
        }
    }

    impl<'res, T: Default + 'static> From<RefMut<'res, Box<dyn Any>>> for ConfigMut<'res, T> {
        fn from(value: RefMut<'res, Box<dyn Any>>) -> Self {
            ConfigMut {
                value,
                _marker: PhantomData,
            }
        }
    }
}
