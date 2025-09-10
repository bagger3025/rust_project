use rust_project::linked_list::LinkedList;

fn main() {
    let mut head = LinkedList::<i32>::new();

    head.push_back(3);
    head.push_front(4);
    head.push_back(5);

    head.print();

    println!(
        "Front is {}, back is {}",
        head.front().unwrap(),
        head.back().unwrap()
    );
    println!("{}", head.len());

    for node in &head {
        println!("node is {node}");
    }

    // 1 4 3 5
    head.push_front(1);
    // 1 4 3
    head.pop_back();
    // 4 3
    head.pop_front();

    head.print();

    for h in &head {
        println!("node={h}");
    }

    let mut ref_iter = (&head).into_iter();
    let first = ref_iter.next();
    let second = ref_iter.next();
    let third = ref_iter.next();

    println!("first = {first:?}, second = {second:?}, third = {third:?}");

    let mut mut_iter = head.iter_mut();
    let first = mut_iter.next();
    let last_ele = mut_iter.next_back();
    let second = mut_iter.next();

    println!("first = {first:?}, last={last_ele:?}, second = {second:?}");

    println!("End of main");
}
