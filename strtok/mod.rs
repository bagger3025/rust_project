use std::marker::PhantomData;

pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn strtok2(_s: &mut &str, _delimiter: char) {}

struct Deserializer<T> {
    _t: PhantomData<T>,
}

struct Deserializer2<T> {
    _t: PhantomData<fn() -> T>,
    // or
    _t2: PhantomData<*const T>,
}

struct Deserializer3<T> {
    _t: PhantomData<fn(T)>,
}

struct Deserializer4<T> {
    _t1: PhantomData<*mut T>,
    // or
    _t2: PhantomData<fn(T) -> T>,
    // or
    _t3: PhantomData<fn() -> T>,
    _t4: PhantomData<fn(T)>,
}

// Invaraint following three types:
//  - *mut T
//  - &mut T
//  - UnsafeCell<T>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x = "hello world";
        // &'a mut &'a      str
        // &   mut &'static str
        fn check_is_static(_: &'static str) {}
        check_is_static(x);
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }

    // #[test]
    // fn it_not_works() {
    //     let mut x = "hello world";
    //     let hello = strtok2(&mut x, ' ');
    //     // assert_eq!(hello, ());
    //     assert_eq!(x, "hello world");
    // }
}

// SUBTYPE
// T is subtype of U <=> T: U
// T is at least as useful as U
//
// 'static: 'a
// Cat: Animal

// Covariance
// fn foo(&'a str) {}
//
// foo(&'a str)
// foo(&'static str)

// ContraVariance
//
// fn foo(bar: Fn(&'a str) -> ()) {
//    bar(&'a str)
// }
//
// foo(fn(&'static str) {})
//  -> fn gets passed into `foo`, and get called with `&'a str`, but expected
//     `&'static str`,
//
// fn foo2(bar: Fn(&'static str) -> ()) {
//    bar(&'static str)
// }
//
// foo2(fn(&'a str) {})
//  -> fn gets passed into `foo`, and get called with `&'static str`, but
//     fn expected `&'a str` and it is okay to get called with `&'static str`,
//     because arguments are covariant ('static can be treated like 'a)
//
// &'static str  vs.  &'a str
//  -> &'static str is more useful(generally applicable)
//  -> 'static is satisfies any 'a
//  -> 'static can be used in any `&'a str`
//  -> 'static <: 'a
//  &'static T <: &'a T
//
// Fn(&'static str)  vs.  Fn(&'a str)
//  -> Fn(&'static str) can be called only with `&'static str`
//  -> Fn(&'a str) can be called with both `&'static str` and `&'a str`
//  -> Therefore, Fn(&'a str) is more useful(generally applicable)
//  -> Fn(&'static str) requires its argument to be maximally useful,
//     which makes the type itself less useful because it has
//     stricter requirements
//  -> Fn(&'a T) <: Fn('static T)
//

// Invariance
// &'a mut T is `covariant` over `'a`, but `invariant` over `T`
//
// fn foo (s: &mut &'a str, x: &'a str) {
//    *s = x;
// }
//
// If &mut T is covaraint over T,
//  -> let mut x: &'static str = "Hello world"; let z = String::new();
//  -> foo(&mut x, &z);
//  -> &mut x is `&mut &'static str`, but `&'static str` can be `&'a str`,
//     because `&mut T` is covariant over T.
//  -> 'a can be the scope of the original function, and can be compiled
//
// However, &mut T is invariant over T,
//  -> &mut x is `&mut &'static str`, and `&'a str` becomes `&'static str`.
//  -> however, `&z` is not `&'static str`, so it does not compile

// &'a mut T is `covariant` over `'a`
// ```Rust
// let y = true; let mut z = &y;
// let x: &'static mut bool = /* &true */;
// z = x; /* &'y mut bool <- &'static mut bool */
// ```
