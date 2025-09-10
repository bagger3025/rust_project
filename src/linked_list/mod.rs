use std::ops::Add;
mod linked_list;
mod linked_list_node;
pub use linked_list::LinkedList;
use linked_list_node::LinkedListNode;
mod linked_list_iter;
pub use linked_list_iter::LinkedListIterator;
mod linked_list_into_iter;
pub use linked_list_into_iter::LinkedListIntoIterator;
mod linked_list_mut_iter;
pub use linked_list_mut_iter::LinkedListMutIterator;

pub struct PlusIteratorStruct<'a, T> {
    iter: &'a mut dyn Iterator<Item = T>,
}

pub trait PlusIterator<T> {
    fn plus_iter(&mut self) -> PlusIteratorStruct<'_, T>;
}

impl<I, T> PlusIterator<T> for I
where
    I: Iterator<Item = T>,
    T: Add<i32, Output = i32>,
{
    fn plus_iter(&mut self) -> PlusIteratorStruct<'_, T> {
        PlusIteratorStruct { iter: self }
    }
}

impl<'a, T> Iterator for PlusIteratorStruct<'a, T>
where
    T: Add<i32, Output = i32>,
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let data = self.iter.next();
        data.map(|d| d + 1)
    }
}
