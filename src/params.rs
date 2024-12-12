use core::{
    any::{Any, TypeId}, cell::{Ref, RefCell, RefMut}, marker::PhantomData, ops::{Deref, DerefMut}
};

use alloc::boxed::Box;
use hashbrown::HashMap;

pub trait ComponentParam {
    type Item<'new>;

    fn retrieve<'r>(config: &'r HashMap<TypeId, RefCell<Box<dyn Any>>>, services: &'r HashMap<TypeId, Box<dyn Any>>) -> Self::Item<'r>;
    fn exists<'r>(config: &'r mut HashMap<TypeId, RefCell<Box<dyn Any>>>, services: &'r mut HashMap<TypeId, Box<dyn Any>>) -> bool;
}

pub struct Config<'a, T: Default + 'static> {
    value: Ref<'a, Box<dyn Any>>,
    _marker: PhantomData<T>,
}

impl<'res, T: Default + 'static> ComponentParam for Config<'res, T> {
    type Item<'new> = Config<'new, T>;

    fn retrieve<'r>(config: &'r HashMap<TypeId, RefCell<Box<dyn Any>>>, _: &'r HashMap<TypeId, Box<dyn Any>>) -> Self::Item<'r> {
        Config {
            value: config.get(&TypeId::of::<T>()).unwrap().borrow(),
            _marker: PhantomData,
        }
    }

    fn exists<'r>(config: &'r mut HashMap<TypeId, RefCell<Box<dyn Any>>>, _: &'r mut HashMap<TypeId, Box<dyn Any>>) -> bool {
        let _ = config.try_insert(TypeId::of::<T>(), RefCell::new(Box::new(T::default())));
        true
    }
}

impl<'res, T: Default + 'static> Deref for Config<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}

pub struct ConfigMut<'a, T: Default + 'static> {
    value: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<T>,
}

impl<'res, T: Default + 'static> ComponentParam for ConfigMut<'res, T> {
    type Item<'new> = ConfigMut<'new, T>;

    fn retrieve<'r>(resources: &'r HashMap<TypeId, RefCell<Box<dyn Any>>>, _: &'r HashMap<TypeId, Box<dyn Any>>) -> Self::Item<'r> {
        ConfigMut {
            value: resources.get(&TypeId::of::<T>()).unwrap().borrow_mut(),
            _marker: PhantomData,
        }
    }

    fn exists<'r>(config: &'r mut HashMap<TypeId, RefCell<Box<dyn Any>>>, _: &'r mut HashMap<TypeId, Box<dyn Any>>) -> bool {
        let _ = config.try_insert(TypeId::of::<T>(), RefCell::new(Box::new(T::default())));
        true
    }
}

impl<T: Default + 'static> Deref for ConfigMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}

impl<T: Default + 'static> DerefMut for ConfigMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.downcast_mut().unwrap()
    }
}

pub struct Service<'a, T: 'static> {
    value: &'a Box<dyn Any>,
    _marker: PhantomData<T>,
}

impl<'res, T: 'static> ComponentParam for Service<'res, T> {
    type Item<'new> = Service<'new, T>;

    fn retrieve<'r>(_: &'r HashMap<TypeId, RefCell<Box<dyn Any>>>, services: &'r HashMap<TypeId, Box<dyn Any>>) -> Self::Item<'r> {
        Service {
            value: services.get(&TypeId::of::<T>()).unwrap(),
            _marker: PhantomData,
        }
    }

    fn exists<'r>(_: &'r mut HashMap<TypeId, RefCell<Box<dyn Any>>>, service: &'r mut HashMap<TypeId, Box<dyn Any>>) -> bool {
        service.contains_key(&TypeId::of::<T>())
    }
}

impl <T: 'static> Deref for Service<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}