use std::{
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread::spawn,
};

fn main() {
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || {
        x.store(true, Ordering::Release);
    });
    let _ty = spawn(move || {
        y.store(true, Ordering::Release);
    });
    let t1 = spawn(move || {
        while !x.load(Ordering::Acquire) {}
        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Ordering::Acquire) {}
        if x.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let z = z.load(Ordering::SeqCst);
    println!("{z}");
    // What are the possible value for z?
    //  - Is 0 possible?
    //    Restrictions: t1 must run "after" tx
    //                  t2 must run "after" ty
    //    Given that..
    //      ..    tx ..    t1 ..
    //      ty t2 tx ..    t1 ..    -> t1 will increment z
    //      .. `y tx `y t2 t1 ..    -> t1 and t2 will increment z
    //      .. `y tx `y    t1 `y t2 -> t2 will increment z
    //    Seems to impossible to have a thread schedule where z == 0
    //    MO(X): false true
    //    MO(Y): false true
    //    t1: X->true, Y->false (regardless ty has run or not, memory system all
    //                           ows to show us the value false)
    //    Acquire-release only guarantees that the same thread that stored the v
    //      alue
    //    All memory ordering is about which things happen before which other co
    //      ncurrent things. If there's not happen-before relationship between t
    //      wo operations, then it's not clear whether one sees the other.
    //
}

// Rust doc:
// SeqCst:
//   Like Acquire/Release/AcqRel with the additional guarantee that all threads
//     see all sequentially consistent operations in the same order.
// SeqCst only really interacts with other SeqCst operations
