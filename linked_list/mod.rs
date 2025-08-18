use std::{fmt::Debug, marker::PhantomData, mem::ManuallyDrop, ops::Add, ptr::null_mut};

#[derive(Debug)]
pub struct LinkedList<T> {
    head: *mut LinkedListNode<T>,
    tail: *mut LinkedListNode<T>,
    len: usize,
}

struct LinkedListNode<T> {
    data: T,
    next: *mut LinkedListNode<T>,
    prev: *mut LinkedListNode<T>,
}

impl<T> LinkedListNode<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            next: null_mut(),
            prev: null_mut(),
        }
    }

    fn new_ptr(data: T) -> *mut Self {
        let node = Box::new(Self::new(data));
        Box::into_raw(node)
    }
}

trait DropPointer {
    fn drop(self);
}

trait MoveBidirection {
    fn move_forward(&mut self);
    fn move_backward(&mut self);
}

trait GetData<T> {
    fn get_data(&self) -> Option<&T>;
}

impl<T> DropPointer for *const LinkedListNode<T> {
    fn drop(self) {
        drop(unsafe { Box::from_raw(self.cast_mut()) });
    }
}

impl<T> MoveBidirection for *mut LinkedListNode<T> {
    fn move_forward(&mut self) {
        if self.is_null() {
            return;
        }

        let node = *self;
        *self = unsafe { (*node).next };
        node.drop();

        if !self.is_null() {
            let data = *self;
            unsafe {
                (*data).prev = null_mut();
            }
        }
    }

    fn move_backward(&mut self) {
        if self.is_null() {
            return;
        }

        let node = *self;
        *self = unsafe { (*node).prev };
        node.drop();
        if !self.is_null() {
            let data = *self;
            unsafe {
                (*data).next = null_mut();
            }
        }
    }
}

impl<T> GetData<T> for *mut LinkedListNode<T> {
    fn get_data(&self) -> Option<&T> {
        match (*self).is_null() {
            true => None,
            false => Some(unsafe { &(**self).data }),
        }
    }
}

impl<T> GetData<T> for *const LinkedListNode<T> {
    fn get_data(&self) -> Option<&T> {
        match (*self).is_null() {
            true => None,
            false => Some(unsafe { &(**self).data }),
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: null_mut(),
            tail: null_mut(),
            len: 0,
        }
    }

    fn push_first(&mut self, data: T) {
        assert!(self.head.is_null());
        assert!(self.tail.is_null());

        let node = LinkedListNode::new_ptr(data);
        self.head = node;
        self.tail = node;
    }

    pub fn push_back(&mut self, data: T) {
        if self.head.is_null() {
            assert!(self.len == 0);
            self.push_first(data);
        } else {
            assert!(self.len > 0);
            let tail = self.tail;
            let new_node = LinkedListNode::new_ptr(data);
            unsafe {
                (*new_node).prev = tail;
                (*tail).next = new_node;
            }
            self.tail = new_node;
        }
        self.len += 1;
    }

    pub fn push_front(&mut self, data: T) {
        if self.head.is_null() {
            assert!(self.len == 0);
            self.push_first(data);
        } else {
            assert!(self.len != 0);
            let head = self.head;
            let new_node = LinkedListNode::new_ptr(data);
            unsafe {
                (*new_node).next = head;
                (*head).prev = new_node;
            }
            self.head = new_node;
        }
        self.len += 1;
    }

    pub fn front(&self) -> Option<&T> {
        self.head.get_data()
    }

    pub fn pop_front(&mut self) {
        self.head.move_forward();

        self.len -= 1;
        if self.head.is_null() {
            self.tail = null_mut();
        }
    }

    pub fn back(&self) -> Option<&T> {
        self.tail.get_data()
    }

    pub fn pop_back(&mut self) {
        self.tail.move_backward();

        self.len -= 1;
        if self.tail.is_null() {
            self.head = null_mut();
        }
    }

    pub fn print(&self)
    where
        T: Debug,
    {
        let mut head = self.head;
        while !head.is_null() {
            unsafe {
                print!("{:?} ", (*head).data);
                head = (*head).next;
            }
        }
        println!();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        println!("LinkedList is dropped!");
        let mut head = self.head;
        while !head.is_null() {
            let cur_head = head;
            head = unsafe { (*head).next };

            drop(unsafe { Box::from_raw(cur_head) });
        }
        self.head = null_mut();
        self.tail = null_mut();
        self.len = 0;
    }
}

pub struct LinkedListIntoIterator<T> {
    node: *const LinkedListNode<T>,
    _unused: PhantomData<LinkedList<T>>,
    _unused_self: ManuallyDrop<LinkedList<T>>,
}

pub struct LinkedListIterator<'a, T> {
    node: Option<&'a LinkedListNode<T>>,
}

impl<'a, T> Iterator for LinkedListIterator<'a, T>
where
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node {
            Some(node) => {
                self.node = unsafe { node.next.as_ref() };
                Some(&node.data)
            }
            None => None,
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = LinkedListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            node: if self.head.is_null() {
                None
            } else {
                unsafe { self.head.as_ref() }
            },
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        let LinkedList { head, .. } = self;

        let ret = LinkedListIntoIterator {
            node: head,
            _unused: PhantomData,
            _unused_self: ManuallyDrop::new(self),
        };
        // self.head = null_mut();
        // self.tail = null_mut();
        // self.len = 0;
        ret
    }
}

trait GetDataInto<T> {
    fn get_data_into(self) -> T;
}

impl<T> GetDataInto<T> for Box<LinkedListNode<T>> {
    fn get_data_into(self) -> T {
        self.data
    }
}

impl<T> Iterator for LinkedListIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node.is_null() {
            None
        } else {
            let node = self.node;
            let node = unsafe { Box::from_raw(node.cast_mut()) };

            self.node = node.next;

            let data = node.get_data_into();

            Some(data)
        }
    }
}

impl<T> Drop for LinkedListIntoIterator<T> {
    fn drop(&mut self) {
        println!("LinkedList Into Iterator dropped!");
        let mut head = self.node;
        while !head.is_null() {
            let node = head;
            head = unsafe { (*head).next };
            drop(unsafe { Box::from_raw(node.cast_mut()) });
        }
    }
}

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
        match data {
            Some(data) => Some(data + 1),
            None => None,
        }
    }
}
