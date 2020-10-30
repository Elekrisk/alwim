use crate::utils::TypeContainer;


pub struct Node<T> {
    extra: TypeContainer,
    data: T
}