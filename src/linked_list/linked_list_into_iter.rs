use super::{LinkedList, LinkedListNode};
use std::{mem::forget, ptr::NonNull};

pub struct LinkedListIntoIterator<T> {
    head: Option<NonNull<LinkedListNode<T>>>,
    tail: Option<NonNull<LinkedListNode<T>>>,
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        let LinkedList { head, tail, .. } = self;

        let ret = LinkedListIntoIterator { head, tail };
        forget(self);
        ret
    }
}

impl<T> Iterator for LinkedListIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            Some(ref mut ptr) => {
                let box_ptr = unsafe { Box::from_raw(ptr.as_mut()) };
                if ptr == self.tail.as_ref().unwrap() {
                    self.head = None;
                    self.tail = None;
                } else {
                    self.head = box_ptr.next();
                }
                Some(box_ptr.get_data_into())
            }
            None => None,
        }
    }
}

impl<T> Drop for LinkedListIntoIterator<T> {
    fn drop(&mut self) {
        println!("LinkedList Into Iterator dropped!");

        let mut head = self.head;
        while let Some(head_ptr) = head {
            let mut node = head_ptr;
            head = unsafe { node.as_ref().next() };
            drop(unsafe { Box::from_raw(node.as_mut()) });
        }
    }
}
