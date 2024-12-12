use alloc::boxed::Box;
use core::marker::PhantomData;
use core::{
    any::{Any, TypeId},
    cell::RefCell,
};
use hashbrown::HashMap;

use super::{params::ComponentParam, Component, IntoComponent};

pub struct FunctionComponent<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

macro_rules! impl_component {
    ($($params:ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<F: FnMut($($params),*), $($params : ComponentParam),*> Component for FunctionComponent<($($params ,)*), F>
        where
            for<'a, 'b> &'a mut F:
                FnMut($($params), *) +
                FnMut($(<$params as ComponentParam>::Item<'b>), *)
        {
            fn run(&mut self, config: &mut HashMap<TypeId, RefCell<Box<dyn Any>>>, services: &mut HashMap<TypeId, Box<dyn Any>>) {
                fn call_inner<$($params),*>(
                    mut f: impl FnMut($($params),*),
                    $($params: $params,)*
                ) {
                    f($($params),*)
                }

                $(
                    if !$params::exists(config, services) {
                        return;
                    }
                )*

                $(
                    let $params = $params::retrieve(config, services);
                )*

                call_inner(&mut self.f, $($params),*);
            }
        }

        impl<F: FnMut($($params),*), $($params : ComponentParam),*> IntoComponent<($($params,)*)> for F
        where
            for<'a, 'b> &'a mut F:
                FnMut($($params), *) +
                FnMut($(<$params as ComponentParam>::Item<'b>), *)
        {
            type Component = FunctionComponent<($($params ,)*), Self>;

            fn into_component(self) -> Self::Component {
                FunctionComponent {
                    f: self,
                    marker: PhantomData
                }
            }
        }
    };
}

impl_component!();
impl_component!(T1);
impl_component!(T1, T2);
impl_component!(T1, T2, T3);
impl_component!(T1, T2, T3, T4);
impl_component!(T1, T2, T3, T4, T5);
