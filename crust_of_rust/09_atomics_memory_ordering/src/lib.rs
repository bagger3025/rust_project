use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    thread::{self},
};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol: stay in S when locked
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                thread::yield_now();
            }
            thread::yield_now();
        }
        // SAFETY: we hold the lock, therefore we can create a mutable reference
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }
}

#[test]
fn too_relaxed() {
    use std::sync::atomic::AtomicUsize;
    use std::thread::spawn;

    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let t1 = spawn(move || {
        let r1 = y.load(Ordering::Relaxed);
        x.store(r1, Ordering::Relaxed);
        r1
    });
    let t2 = spawn(move || {
        let r2 = x.load(Ordering::Relaxed);
        y.store(42, Ordering::Relaxed);
        r2
    });

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
    // r1 == r2 == 42
    // MO(x): 0 42
    // MO(y): 0 42
}

// From C++
// Acquire: A load operation with this memory order performs the acuiqre operati
//          on on the affected memory location: no reads or writes in the curren
//          t thread can be reordered before this load. All writes in other thre
//          ads that release the same atomic variable are visible in the current
//          thread.
// Release: A store operatiion with this memory order performs the release opera
//          tion. No reads or writes in the current thread can be reordered afte
//          r this store.
//
// From Rust:
// Release: When coupled with a store, all previous operations become ordered be
//          fore any load of this value with Acquire (or stronger) ordering.

// Fences
// Every fence synchronizes with another fence if there are memory operations be
//   fore and after the fences that happen to synchronize
