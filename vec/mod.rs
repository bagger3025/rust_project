use std::{
    alloc::{self, Layout},
    mem::{self, ManuallyDrop},
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    slice,
};

struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

impl<T> RawVec<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "TODO: Implement ZST support");
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }

    fn ptr_mut(&mut self, idx: usize) -> *mut T {
        unsafe { self.ptr.as_ptr().add(idx) }
    }

    fn ptr(&self, idx: usize) -> *const T {
        unsafe { self.ptr.as_ptr().add(idx) }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow since self.cap <+ isize::MAX
            let new_cap = 2 * self.cap;

            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure that the new allocaation doesn't exceed `isize::MAX` bytes.
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

pub struct MyVec<T> {
    buf: RawVec<T>,
    len: usize,
}

unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}

impl<T> MyVec<T> {
    fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn new() -> Self {
        Self {
            buf: RawVec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::write(self.buf.ptr_mut(self.len), elem);
        }

        // Can't fail, we'll OOM first.
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.buf.ptr_mut(self.len))) }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        // Note: `<=` because it's valid to insert after everything
        // which would be equivalent to push.
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.cap() {
            self.buf.grow();
        }
        unsafe {
            ptr::copy(
                self.buf.ptr_mut(index),
                self.buf.ptr_mut(index + 1),
                self.len - index,
            );
            ptr::write(self.buf.ptr_mut(index), elem);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        // Note `<` because it's not valid to remove after everything
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.buf.ptr_mut(index));
            ptr::copy(
                self.buf.ptr_mut(index + 1),
                self.buf.ptr_mut(index),
                self.len - index,
            );
            result
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        // calling `pop` is not needed if T : !Drop
        // We can ask Rust if T needs_drop and omit the calls
        // However in practice LLVM is really good at
        // removing side-effect free code like this,
        // so wouldn't bother unless notice it's not being stripped.
        while let Some(_) = self.pop() {}
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.buf.ptr(0), self.len) }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.buf.ptr_mut(0), self.len) }
    }
}

pub struct IntoIter<T> {
    buf: RawVec<T>,
    start: *const T,
    end: *const T,
}

impl<T> IntoIterator for MyVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        // Make sure not to drop Vec since that would free the buffer
        let vec = ManuallyDrop::new(self);

        // Can't destructure Vec since it's Drop
        let buf = unsafe { ptr::read(&vec.buf) };
        let cap = buf.cap;
        let len = vec.len;

        IntoIter {
            start: buf.ptr(0),
            end: if cap == 0 { buf.ptr(0) } else { buf.ptr(len) },
            buf,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}
