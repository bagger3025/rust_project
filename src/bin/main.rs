#![allow(unused_macros, dead_code)]
use std::{fmt::Debug, marker::PhantomData};

use lifetime::strsplit::StrSplit;
use rust_project::linked_list::{LinkedList, PlusIterator};

#[derive(Debug)]
struct MyStruct<T> {
    data: i32,
    _unused: PhantomData<T>,
}

impl<T> MyStruct<T> {
    fn push(&mut self, data: T)
    where
        T: Debug,
    {
        println!("{data:?}");
    }
}

fn main() {
    let mut head = LinkedList::<i32>::new();

    head.push_back(3);
    head.push_front(4);
    head.push_back(5);

    head.print();
    println!("");

    println!(
        "Front is {}, back is {}",
        head.front().unwrap(),
        head.back().unwrap()
    );
    println!("{}", head.len());

    for data in (&head).into_iter().plus_iter() {
        println!("data = {data}");
    }

    for data in head.into_iter().plus_iter() {
        println!("data = {data}");
    }

    println!("End of main");

    let x = StrSplit::new("a", " ");
    for t in x {
        println!("{}", t);
    }
}
