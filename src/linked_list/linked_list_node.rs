use std::ptr::NonNull;

struct LinkedNextPrev<T> {
    next: Option<NonNull<LinkedListNode<T>>>,
    prev: Option<NonNull<LinkedListNode<T>>>,
}

impl<T> LinkedNextPrev<T> {
    fn new() -> Self {
        Self {
            next: None,
            prev: None,
        }
    }
}

impl<T> Drop for LinkedNextPrev<T> {
    fn drop(&mut self) {
        if let Some(mut next) = self.next {
            unsafe { next.as_mut() }.ptr.prev = self.prev;
        }
        if let Some(mut prev) = self.prev {
            unsafe { prev.as_mut() }.ptr.next = self.next;
        }
    }
}

/// Node for Double Linked List
pub struct LinkedListNode<T> {
    data: T,
    ptr: LinkedNextPrev<T>,
}

impl<T> LinkedListNode<T> {
    /// Create new Double Linked List Node
    /// Will only be used in adding element in Linked List
    pub(super) fn new(data: T) -> Self {
        Self {
            data,
            ptr: LinkedNextPrev::new(),
        }
    }

    pub(super) fn new_ptr(data: T) -> NonNull<Self> {
        let node = Box::new(Self::new(data));

        // SAFETY: Box::into_raw never retruns null pointer
        NonNull::new(Box::into_raw(node)).unwrap()
    }

    pub(super) fn connect_next(&mut self, other: &mut Self) {
        assert!(self.ptr.next.is_none());
        assert!(other.ptr.prev.is_none());

        self.ptr.next = Some(NonNull::from_mut(other));
        other.ptr.prev = Some(NonNull::from_mut(self));
    }

    #[inline]
    pub(super) fn get_data(&self) -> &T {
        &self.data
    }

    #[inline]
    pub(super) fn get_data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    #[inline]
    pub(super) fn get_data_into(self) -> T {
        self.data
    }

    #[inline]
    pub(super) fn next(&self) -> Option<NonNull<Self>> {
        self.ptr.next
    }

    #[inline]
    pub(super) fn prev(&self) -> Option<NonNull<Self>> {
        self.ptr.prev
    }
}
