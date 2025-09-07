fn main() {
    println!("Hello, world!");
    // Function Item
    let x = bar::<i32>;
    println!("x sizeofval = {}", std::mem::size_of_val(&x));
    baz(bar::<i32>);
    baz(bar::<u32>);
    // quox(bar::<i32>);

    let mut z = String::from("t");
    println!("{}", z);
    let mut f = move |x: u32| {
        println!("{}", z);
        x
    };
    // baz(f);
    // quox(&mut f);
    let f: &mut dyn FnMut(u32) -> u32 = &mut f;
    quox(f);

    let x = || 0;
    // const_foo(make_zero);
}

const fn make_zero() -> i32 {
    0
}

// const fn const_foo<F: ~const FnOnce() -> i32>(f: F) {
//     f();
// }

fn bar<T>(_: u32) -> u32 {
    3
}

/// Function Pointer, Function item can be coerced to Function Pointer
/// Function pointer/item has no state, standalone, just chunks of code
///                       no reference any memory, no lifetime associated
///                       no care about "self", nothing to mutate, nothing to mo
///                         ve
fn baz(f: fn(u32) -> u32) {
    println!("Size of val = {}", std::mem::size_of_val(&f));
    println!("Returned: {}", f(13))
}

fn quox<F>(mut f: F)
where
    F: FnMut(u32) -> u32, // -> this is trait
{
    f(13);
}

// From Rustdoc
//   Since `Fn` and `FnMut` are subraits of `FnOnce`, any instance of `Fn` or `F
//     nMut` can be used where `FnOnce` is expected

fn make_fn() -> impl Fn() {
    let z = String::new();
    move || {
        println!("{}", z);
    }
}

/// Box<dyn Fn()> not implement Fn() -> Previously
fn dyn_fn(f: Box<dyn Fn()>) {}
