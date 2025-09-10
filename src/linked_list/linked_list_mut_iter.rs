use std::{marker::PhantomData, ptr::NonNull};

use super::{LinkedList, LinkedListNode};

pub struct LinkedListMutIterator<'a, T> {
    head: Option<NonNull<LinkedListNode<T>>>,
    tail: Option<NonNull<LinkedListNode<T>>>,
    _marker: PhantomData<&'a mut LinkedListNode<T>>,
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;

    type IntoIter = LinkedListMutIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListMutIterator {
            head: self.head,
            tail: self.tail,
            _marker: PhantomData,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn iter_mut<'a>(&'a mut self) -> LinkedListMutIterator<'a, T> {
        self.into_iter()
    }
}

impl<'a, T> Iterator for LinkedListMutIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            Some(mut ptr) => {
                let ptr = unsafe { ptr.as_mut() };

                if self.head == self.tail {
                    self.head = None;
                    self.tail = None;
                } else {
                    self.head = ptr.next();
                }

                Some(ptr.get_data_mut())
            }
            None => None,
        }
    }
}

impl<'a, T> DoubleEndedIterator for LinkedListMutIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.tail {
            Some(mut ptr) => {
                let ptr = unsafe { ptr.as_mut() };

                if self.head == self.tail {
                    self.head = None;
                    self.tail = None;
                } else {
                    self.tail = ptr.prev();
                }

                Some(ptr.get_data_mut())
            }
            None => None,
        }
    }
}
