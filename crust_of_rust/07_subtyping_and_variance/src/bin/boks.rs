use std::iter::Empty;

// use rust_project::boks::Boks;
use boks::{Boks, EmptyIterator, Oisann};

fn main() {
    let x = 42;
    let b = Boks::ny(x);
    println!("{:?}", *b);

    let mut y = 42;
    let b = Boks::ny(&mut y);

    println!("{:?}", y);

    // The following not compiles
    // let mut z = 42;
    // let b = Boks::ny(Oisann(&mut z));
    // let b = Box::new(Oisann(&mut z));
    // println!("{:?}", z);

    let s = String::new();
    let mut z: Boks<&str> = Boks::ny(&*s);
    // let mut z: Box<&str> = Box::new(&*s);
    let static_str: &'static str = "Hello, World!";
    let new_z: Boks<&'static str> = Boks::ny(static_str);
    // let new_z: Box<&'static str> = Box::new(static_str);

    z = new_z;

    let mut e = Empty::default();

    let mut x = 42;
    let mut o = Some(Oisann(&mut x));
    {
        o = e.next();
    }

    println!("{:?}", x);
}

fn foo<'a>(a: &'a mut i32, b: &'a mut i32) {
    let mut my_empty: EmptyIterator<Oisann<&'a mut i32>> = EmptyIterator::default();
    let mut o: Option<Oisann<&'a mut i32>> = Some(Oisann(b));

    o = my_empty.next();
    drop(o);

    println!("{:?}", a);
    let _ = my_empty.next();
}
