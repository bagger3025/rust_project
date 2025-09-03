#![feature(dropck_eyepatch)]

use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub struct Boks<T> {
    p: NonNull<T>,
    _t: PhantomData<T>,
}

// may_dangle is promise for "NOT ACCESSING" T
// we need to tell the compiler that we're going to "DROP" the T.
unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        // SAFETY: p was constructed from a Box in the first place, and has not
        // been freed, otherwise since self still exists (otherwise, drop could
        // not be called)
        unsafe {
            drop(Box::from_raw(self.p.as_mut()));
            // std::ptr::drop_in_place(self.p); -> Would drop T, but not Box
        }
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            // SAFETY: Box never creates a null pointer
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
            _t: PhantomData,
        }
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: is valid since it was constructed from a valid T, and turned
        // into a pointer through Box which creates aligned pointers,
        // and hasn't been freed, since self is alive
        unsafe { self.p.as_ref() }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: is valid since it was constructed from a valid T, and turned
        // into a pointer through Box which creates aligned pointers,
        // and hasn't been freed, since self is alive
        // Also, since we have &mut self, no other mutable reference has been
        // given out to p
        unsafe { self.p.as_mut() }
    }
}

pub struct Oisann<T>(pub T)
where
    T: Debug;

impl<T> Drop for Oisann<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

pub struct EmptyIterator<T>(PhantomData<T>);

impl<T> Default for EmptyIterator<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Iterator for EmptyIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
