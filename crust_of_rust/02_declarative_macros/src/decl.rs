#![allow(unused_macros)]

#[macro_export]
macro_rules! avec {
    ($($element:expr),* $(,)?) => {{
        let count = $crate::count![@COUNT; $($element),*];
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(count);
        $(vs.push($element);)*
        vs
    }};

    ($element:expr; $count:expr) => {{
        // let count = $count;
        // let val = $element;

        let mut vs = Vec::new();
        vs.resize($count, $element);
        // vs.extend(::std::iter::repeat($element).take(count));
        // for _ in 0..$count {
        //     vs.push(val.clone());
        // }
        vs
    }};

}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[
            $($crate::count![@SUBST; $element]),*
        ])
    };

    (@SUBST; $element:expr) => {
        ()
    };

}

// avec!();
// avec![];
// avec! {}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec!();

    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];

    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];

    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trailing() {
    let x: Vec<u32> = avec![42, 43,];

    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn count() {
    let x: Vec<u32> = avec![42;10];

    assert!(!x.is_empty());
    assert_eq!(x.len(), 10);
    for i in 0..10 {
        assert_eq!(x[i], 42);
    }
}

#[test]
fn count_complicated() {
    let mut v = Some(42);
    let x: Vec<u32> = avec![v.take().unwrap();10];

    assert!(!x.is_empty());
    assert_eq!(x.len(), 10);
    for i in 0..10 {
        assert_eq!(x[i], 42);
    }
}

/// Just a struct for compiling test
///
/// ```compile_fail
/// let x: Vec<u32> = rust_project::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailure;
