use std::{fmt::Debug, marker::PhantomData};

mod lifetimes;
mod vec;

use lifetimes::strsplit::StrSplit;
use linked_list::{LinkedList, PlusIterator};

mod linked_list;
// use std::{borrow::Borrow, cell::RefCell, rc::Rc};

// struct LinkdList {
//     head: Option<Rc<RefCell<Box<LinkedListNode>>>>,
//     tail: Option<Rc<RefCell<Box<LinkedListNode>>>>,
// }

// struct LinkedListNode {
//     data: i32,
//     next: Option<Rc<RefCell<Box<LinkedListNode>>>>,
// }

// impl LinkedListNode {
//     fn new(val: i32) -> LinkedListNode {
//         LinkedListNode {
//             data: val,
//             next: None,
//         }
//     }
// }

// impl LinkedList {
//     fn new() -> LinkedList {
//         LinkedList {
//             head: None,
//             tail: None,
//         }
//     }

//     fn insert(&mut self, val: i32) {
//         match &self.tail {
//             Some(tail) => {
//                 let node = Rc::new(RefCell::new(Box::new(LinkedListNode::new(val))));
//                 tail.clone().as_ref().borrow_mut().next = Some(Rc::clone(&node));
//                 self.tail = Some(Rc::clone(&node));
//             }
//             None => {
//                 let node = Rc::new(RefCell::new(Box::new(LinkedListNode::new(val))));

//                 self.head = Some(Rc::clone(&node));
//                 self.tail = Some(Rc::clone(&node));
//             }
//         }
//     }

//     fn delete(&mut self, val: i32) {
//         if self.head.is_none() {
//             return;
//         }

//         let mut node = self.head.clone();

//         if self.head.as_ref().unwrap().as_ref().borrow().data == val {
//             let next_node = self.head.as_ref().unwrap().as_ref().borrow().next.clone();
//             self.head = next_node;
//             if self.head.is_none() {
//                 self.tail = None;
//             }
//         }

//         while node.is_some() {
//             let next_option = node.as_ref().unwrap().as_ref().borrow().next.clone();
//             let next_node = next_option.as_ref().unwrap().as_ref().borrow();
//             println!("{:?}, {:?}", &next_node.data, &val);
//             if next_node.data == val {
//                 let next_linked_node = next_node.next.clone();
//                 drop(next_node);
//                 node.as_ref().unwrap().as_ref().borrow_mut().next = next_linked_node.clone();
//                 if next_linked_node.is_none() {
//                     self.tail = node;
//                 }
//                 return;
//             }
//             drop(next_node);
//             node = next_option;
//         }
//     }

//     fn print(&self) {
//         let mut rc;
//         let mut node = self.head.clone();

//         while node.is_some() {
//             print!("{:?} ", &node.as_ref().unwrap().as_ref().borrow().data);
//             rc = node.as_ref().unwrap().clone();
//             node = rc.as_ref().borrow().next.clone();
//         }
//         println!();
//     }
// }

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
    let s = String::from("s");

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
