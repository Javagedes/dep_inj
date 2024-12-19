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

use crate::{unsafe_storage::UnsafeStorageCell, MetaData};
use sdk::component::Storage;

use super::{params::ComponentParam, Component, IntoComponent};

type ComponentParamItem<'w, 'state, P> = <P as ComponentParam>::Item<'w, 'state>;

/// A [Component] implementation for a function whose parameters all implement [ComponentParam].
#[allow(private_bounds)]
pub struct FunctionComponent<Marker, Func>
where
    Func: ComponentParamFunction<Marker>,
{
    func: Func,
    param_state: Option<<Func::Param as ComponentParam>::State>,
    metadata: MetaData,
    marker: PhantomData<fn() -> Marker>,
}

impl<Marker, Func> Component for FunctionComponent<Marker, Func>
where
    Marker: 'static,
    Func: ComponentParamFunction<Marker>,
{
    /// Runs the component if all parameters are retrievable from storage.
    ///
    /// ## Safety
    ///
    /// - Each parameter must properly register its access type.
    unsafe fn run_unsafe(&mut self, storage: UnsafeStorageCell) -> bool {
        let param_state = self.param_state.as_mut().expect("Should Exist");
        if !Func::Param::validate(param_state, storage) {
            return false;
        }

        let param_value = Func::Param::retrieve(param_state, storage);

        self.func.run(param_value);

        true
    }

    /// Returns the metadata of the component.
    fn metadata(&self) -> &MetaData {
        &self.metadata
    }

    /// One time initialization of the component. Should set access requirements.
    fn initialize(&mut self, storage: &mut Storage) {
        self.param_state = Some(Func::Param::initialize(storage, &mut self.metadata));
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
            param_state: None,
            metadata: MetaData::new::<F>(),
            marker: PhantomData,
        }
    }
}

/// An internal trait allows the Component implementation for FunctionComponent to be generic over a function with
/// any amount of parameters. The macro [impl_component_param_function] implements this trait for the different
/// amounts of parameters. This way we can more easily make Component implementation for functions more complex without
/// having to mirror that complexity to the macro.
trait ComponentParamFunction<Marker>: Send + Sync + 'static {
    type Param: ComponentParam;

    fn run(&mut self, param_value: ComponentParamItem<Self::Param>);
}

macro_rules! impl_component_param_function {
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

impl_component_param_function!();
impl_component_param_function!(T1);
impl_component_param_function!(T1, T2);
impl_component_param_function!(T1, T2, T3);
impl_component_param_function!(T1, T2, T3, T4);
impl_component_param_function!(T1, T2, T3, T4, T5);
