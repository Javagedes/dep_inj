pub mod params {
    extern crate alloc;

    use alloc::boxed::Box;
    use core::{any::Any, cell::{Ref, RefCell}, marker::PhantomData, ops::Deref};

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

    impl <'res, T: Default + 'static> From<&'res RefCell<Box<dyn Any>>> for Config<'res, T> {
        fn from(value: &'res RefCell<Box<dyn Any>>) -> Self {
            Config {
                value: value.borrow(),
                _marker: PhantomData,
            }
        }
    }

    pub struct Service<'a, T: 'static> {
        value: &'a Box<dyn Any>,
        _marker: PhantomData<T>,
    }

    impl <T: 'static> Deref for Service<'_, T> {
        type Target = T;
    
        fn deref(&self) -> &T {
            self.value.downcast_ref().unwrap()
        }
    }

    impl <'res, T: 'static> From<&'res Box<dyn Any>> for Service<'res, T> {
        fn from(value: &'res Box<dyn Any>) -> Self {
            Service {
                value,
                _marker: PhantomData,
            }
        }
    }
}