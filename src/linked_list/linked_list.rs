use super::linked_list_node::LinkedListNode;
use std::{fmt::Debug, ptr::NonNull};

#[derive(Debug)]
pub struct LinkedList<T> {
    pub(super) head: Option<NonNull<LinkedListNode<T>>>,
    pub(super) tail: Option<NonNull<LinkedListNode<T>>>,
    pub(super) len: usize,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList::<T>::default()
    }

    /// Pushs the data in the back of the Double Linked List
    pub fn push_back(&mut self, data: T) {
        if let Some(ref mut tail) = self.tail {
            assert!(self.len > 0);
            let mut new_node = LinkedListNode::new_ptr(data);

            // SAFETY: self is mutably borrowed, so this is the only one who is
            //         accessing this linked list. Therefore it is safe to borro
            //         w them as mut reference.
            unsafe { tail.as_mut() }.connect_next(unsafe { new_node.as_mut() });

            *tail = new_node;
            self.len += 1;
        } else {
            self.push_first(data);
        }
    }

    /// Pushs the data in the front of the Double Linked List
    pub fn push_front(&mut self, data: T) {
        if let Some(ref mut head) = self.head {
            assert!(self.len > 0);
            let mut new_node = LinkedListNode::new_ptr(data);

            // SAFETY: self is mutably borrowed, so this is the only one who is
            //         accessing this linked list. Therefore it is safe to borro
            //         w them as mut reference
            unsafe { new_node.as_mut() }.connect_next(unsafe { head.as_mut() });

            *head = new_node;
            self.len += 1;
        } else {
            self.push_first(data);
        }
    }

    /// Returns the front data in the Double Linked List
    /// Only returning reference of the data
    pub fn front(&self) -> Option<&T> {
        self.head.map(|ref ptr| unsafe { ptr.as_ref() }.get_data())
    }

    /// Drops the front data in the Double Linked List
    /// Does not do anything if it has no data
    pub fn pop_front(&mut self) {
        if self.len == 0 {
            return;
        }

        let Some(mut ptr) = self.head else {
            unreachable!()
        };

        self.head = unsafe { ptr.as_ref() }.next();

        // SAFETY: self is mutably borrowed, so only this variable holds the ptr
        //         . Dropping the `ptr` makes head's prev None.
        drop(unsafe { Box::from_raw(ptr.as_mut()) });

        self.len -= 1;
        if self.head.is_none() {
            assert_eq!(self.len, 0);
            self.tail = None;
        }
    }

    /// Returns the back data in the Double Linked List
    /// Only returning reference of the data
    pub fn back(&self) -> Option<&T> {
        self.tail.map(|ref ptr| unsafe { ptr.as_ref() }.get_data())
    }

    /// Drops the back data in the Double Linked List
    pub fn pop_back(&mut self) {
        if self.len == 0 {
            return;
        }

        let Some(mut ptr) = self.tail else {
            unreachable!();
        };
        self.tail = unsafe { ptr.as_ref() }.prev();

        // SAFETY: self is mutably borrowed, so only this variable holds the ptr
        //         . Dropping the `ptr` makes tail's next None.
        drop(unsafe { Box::from_raw(ptr.as_mut()) });

        self.len -= 1;
        if self.tail.is_none() {
            assert_eq!(self.len, 0);
            self.head = None;
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Prints the datas in the Double Linked List.
    /// Prints the data in one line, and prints newline in the last.
    /// T should implement Debug.
    pub fn print(&self)
    where
        T: Debug,
    {
        let mut head = self.head;
        while let Some(head_ptr) = head {
            unsafe {
                print!("{:?} ", head_ptr.as_ref().get_data());
                head = head_ptr.as_ref().next();
            }
        }
        println!();
    }
}

impl<T> LinkedList<T> {
    /// Always called inside. Head and Tail must be None to call this method
    fn push_first(&mut self, data: T) {
        assert!(self.head.is_none());
        assert!(self.tail.is_none());
        assert_eq!(self.len, 0);

        let node = LinkedListNode::new_ptr(data);
        self.head = Some(node);
        self.tail = Some(node);
        self.len = 1;
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        println!("LinkedList is dropped!");

        let mut head = self.head;
        while let Some(mut head_ptr) = head {
            head = unsafe { head_ptr.as_ref() }.next();

            drop(unsafe { Box::from_raw(head_ptr.as_mut()) });
        }
        self.head = None;
        self.tail = None;
        self.len = 0;
    }
}
