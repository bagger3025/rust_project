#![allow(dead_code, unused_variables)]

use std::{future::Future, pin::pin};

fn main() {
    println!("Hello, World!");

    let x = pin!(foo1());
    let y = x.poll();
}

async fn foo1() -> usize {
    3
}

/// Equivalent to `async fn foo1() {}`
fn foo2() -> impl Future<Output = usize> {
    async {
        yield;
        3
    }
}
