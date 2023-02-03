use crate::math::point::Point;

pub trait Event {
    fn clone(&self) -> Self
    where
        Self: Sized;
}

#[derive(Clone)]
pub struct PointerClick(pub Point);
impl Event for PointerClick {
    fn clone(&self) -> Self
    where
        Self: Sized,
    {
        Clone::clone(&self)
    }
}

#[derive(Clone)]
pub struct OnClick(pub Point);
impl Event for OnClick {
    fn clone(&self) -> Self
    where
        Self: Sized,
    {
        Clone::clone(&self)
    }
}
