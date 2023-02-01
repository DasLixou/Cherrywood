use std::{marker::PhantomData, rc::Rc, sync::Mutex};

use crate::{access::Access, app::App, system_param::SystemParam};

pub trait IntoDescribedSystem<Out, Params> {
    type System: DescribedSystem<Out> + 'static;

    fn into_described(self) -> Self::System;
}

pub trait DescribedSystem<Out> {
    fn initialize(&mut self);

    fn run(&mut self, app: &mut App) -> Out;
}

pub type BoxedDescribedSystem<Out = ()> = Rc<Mutex<dyn DescribedSystem<Out>>>;

pub struct FunctionSystem<F, Params> {
    system: F,
    access: Access,
    phantom: PhantomData<Params>,
}

impl<F, Out, Params: SystemParam + 'static> IntoDescribedSystem<Out, Params> for F
where
    F: SystemParamFunction<Out, Params> + 'static,
{
    type System = FunctionSystem<F, Params>;

    fn into_described(self) -> Self::System {
        FunctionSystem {
            system: self,
            access: Access::new(),
            phantom: PhantomData,
        }
    }
}

impl<F, Out, Params: SystemParam> DescribedSystem<Out> for FunctionSystem<F, Params>
where
    F: SystemParamFunction<Out, Params> + 'static,
{
    fn initialize(&mut self) {
        F::initialize(&mut self.access);
    }

    fn run(&mut self, app: &mut App) -> Out {
        SystemParamFunction::run(&mut self.system, app)
    }
}

pub(crate) trait SystemParamFunction<Out, Params: SystemParam>: 'static {
    fn initialize(access: &mut Access)
    where
        Self: Sized;

    fn run(&mut self, app: &mut App) -> Out;
}
