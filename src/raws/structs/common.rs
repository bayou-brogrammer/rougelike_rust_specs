use std::any::Any;

use super::Renderable;

pub trait BaseRawComponent: Clone {
    fn name(&self) -> String;
    fn renderable(&self) -> Option<Renderable>;

    // fn to_struct<U>(self) -> U;
    fn as_any(&self) -> &dyn Any;
}
