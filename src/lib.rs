use std::rc::Rc;

pub trait Component<T> {
    fn get_component(&self) -> Rc<T>;
}

pub trait Entity {
    fn get<T>(&self) -> Rc<T> where Self: Component<T>;
}

impl<E: ?Sized> Entity for E {
    fn get<T>(&self) -> Rc<T> where Self: Component<T> {Component::<T>::get_component(self)}
}