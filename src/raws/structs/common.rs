use core::fmt::Debug;
use std::any::Any;

use super::Renderable;

pub trait BaseRawComponent {
    fn name(&self) -> String;
    fn renderable(&self) -> Option<Renderable>;
    fn as_any(&self) -> &dyn Any;

    // fn to_struct<U>(self) -> U;
}

impl Debug for dyn BaseRawComponent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BaseRawComponent{{{}}}", self.name())
    }
}
