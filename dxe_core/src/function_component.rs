//! A Module representing a [Component] implementation for a function whose parameters implement [ComponentParam].
//! 
//! All code in this module does not need to be used directly by the user. It exists to be able to provide a blanket
//! implementation for all functions whose parameters implement [ComponentParam]. This ranges from functions with no
//! parameters (such as `fn my_component()`) to functions with multiple parameters (such as
//! `fn my_component(data: Config<i32>, data2: Config<f32>)`).
//! 
//! The flow of how this module works is as follows:
//! 
//! [impl_component] is a macro that generates implementations for [ComponentParamFunction] for all functions whose
//! parameters only implement [ComponentParam]. This allows us to write all remaining code and logic outside of the
//! macro, allowing us to call functions on the parameters without knowing the exact number of parameters.
//! 
//! 
use core::marker::PhantomData;

use crate::Storage;

use super::{params::ComponentParam, Component, IntoComponent};

pub type ComponentParamItem<'w, P> = <P as ComponentParam>::Item<'w>;

/// A [Component] implementation for a function whose parameters implement [ComponentParam].
pub struct FunctionComponent<Marker, F>
where 
    F: ComponentParamFunction<Marker>,
{
    func: F,
    marker: PhantomData<fn() -> Marker>,
}

impl<Marker, F> Component for FunctionComponent<Marker, F>
where 
    Marker: 'static,
    F: ComponentParamFunction<Marker>,
{
    fn run(&mut self,
        storage: &mut Storage,
    ) -> bool {
        if !F::Param::exists(storage) {
            return false;
        }

        let param_value = F::Param::retrieve(storage);

        self.func.run(param_value);

        true
    }
}

impl<Marker, F> IntoComponent<Marker> for F
where 
    Marker: 'static,
    F: ComponentParamFunction<Marker>,
{
    type Component = FunctionComponent<Marker, F>;
    fn into_component(self) -> Self::Component {
        FunctionComponent {
            func: self,
            marker: PhantomData,
        }
    }
}

pub trait ComponentParamFunction<Marker>: Send + Sync + 'static {
    type Param: ComponentParam;

    fn run(
        &mut self,
        param_value: ComponentParamItem<Self::Param>,
    );
}

macro_rules! impl_component {
    ($($param:ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<Func, $($param : ComponentParam),*> ComponentParamFunction<fn($($param,)*)> for Func
        where
            Func: Send + Sync + 'static,
            for<'a, 'b> &'a mut Func:
                FnMut($($param), *) +
                FnMut($(ComponentParamItem<$param>),*)
        {
            type Param = ($($param,)*);
            fn run(&mut self, param_value: ComponentParamItem<($($param,)*)>) {
                fn call_inner<$($param),*>(
                    mut f: impl FnMut($($param),*),
                    $($param: $param,)*
                ) {
                    f($($param),*)
                }
                let ($($param,)*) = param_value;
                call_inner(self, $($param),*);
            }
        }
    }
}

impl_component!();
impl_component!(T1);
impl_component!(T1, T2);
impl_component!(T1, T2, T3);
impl_component!(T1, T2, T3, T4);
impl_component!(T1, T2, T3, T4, T5);
