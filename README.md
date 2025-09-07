# Rust Project

## Double Linked List

| ./src/linked_list

Implemented alone with rust std doc, Rustonomicon, etc.

## Implementing Vec

| ./src/vec

Following: [The Rustonomicon](https://doc.rust-lang.org/nomicon/vec/vec.html)

## Crust of Rust

| ./crust_of_rust

- [Lifetime Annotations](https://youtu.be/rAl-9HwD858)
- [Declarative Macros](https://youtu.be/q6paRBbLgNw)
- [Iterators](https://youtu.be/yozQ9C69pNs)
- [Smart Pointers and Interior Mutability](https://youtu.be/8O0Nt9qY_vo)
- [Channels](https://youtu.be/b4mS5UPHh20)
- [Sorting Algorithms](https://youtu.be/h4RkCyJyXmM)
- [Subtyping and Variance](https://youtu.be/iVYWDIW71jk)
- [The Drop Check](https://youtu.be/TJOFSMpJdzg)
  - [Rustonomicon: Drop Check](https://doc.rust-lang.org/nomicon/dropck.html)
- [Atomics and Memory Ordering](https://youtu.be/rMGWeSjctlY)
  - [Google ThreadSanitizer](https://github.com/google/sanitizers/wiki/ThreadSanitizerAlgorithm)
  - [Loom](https://github.com/tokio-rs/loom)
  - [cppreference: memory order](https://en.cppreference.com/w/cpp/atomic/memory_order.html)
  - [Rust std doc Ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html)
  - [Rust std doc atomic Fence](https://doc.rust-lang.org/std/sync/atomic/fn.fence.html)
- [Dispatch and Fat Pointers](https://youtu.be/xcygqF5LVmM?si=j1T3F2ojryqc_M24)
  - [Auto traits](https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits)
  - [Rust Reference: Dynamically Sized Types](https://doc.rust-lang.org/reference/dynamically-sized-types.html)
  - [Rust Reference: Dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)
  - [Rust std doc: Module Any](https://doc.rust-lang.org/std/any/index.html)
- [Async and Await](https://youtu.be/ThjvMReOXYM)
  - [The async book](https://rust-lang.github.io/async-book/)
  - [Tokio Mini Redis](https://github.com/tokio-rs/mini-redis/)
  - [Tokio rs](https://docs.rs/tokio/latest/tokio/)
  - [Tokio task spawn_blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)
  - [Rust doc Futures crate](https://docs.rs/futures/latest/futures/)
- [Functions, Closures, and their Traits](https://youtu.be/dHkzSZnYXmk)
  - [Rust 1.35.0: Fn* Closure traits implemented for Box<dyn Fn*>](https://blog.rust-lang.org/2019/05/23/Rust-1.35.0/#fn-closure-traits-implemented-for-box-dyn-fn)
  - [Rust 1.61.0: More Capabilities for Const Fn](https://blog.rust-lang.org/2022/05/19/Rust-1.61.0/#more-capabilities-for-const-fn)
