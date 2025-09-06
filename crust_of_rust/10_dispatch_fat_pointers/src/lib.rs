pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn strlen_dyn(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

/// Equivalent with `strlen(s: impl AsRef<str>)`
pub fn strlen2<S>(s: S) -> usize
where
    S: AsRef<str>,
{
    s.as_ref().len()
}

pub fn foo() {
    strlen("Hello World!"); // &'static str
    strlen(String::from("Hello World!")); // String: AsRef<str>
}

pub trait Hei {
    type Name;

    fn hei(&self);

    fn weird()
    where
        Self: Sized,
    {
    }
}

impl Hei for &str {
    type Name = ();
    fn hei(&self) {
        println!("hei {}", self);
    }
}
impl Hei for String {
    type Name = ();
    fn hei(&self) {
        println!("hei {}", self);
    }
}

impl Hei for str {
    type Name = ();
    fn hei(&self) {
        println!("Hei {}", self);
    }
}

pub fn heifoo() {
    "J".hei();
}

pub fn bar(h: impl Hei) {
    h.hei();
}

pub fn bar_dyn(h: &dyn Hei<Name = ()>) {
    h.hei();
}

pub trait HeiAsRef: Hei<Name = ()> + AsRef<str> {}

pub fn baz(_h: &dyn HeiAsRef) {}

pub fn heifoo2() {
    for h in vec!["J", "Jon"] {
        h.hei();
    }

    // heibar2(&["J", "Jon"]);
    // heibar2(&["J", String::from("Jon")]);

    // let v: Vec<str> = vec![];
}

// pub fn heibar2<H>(s: &[dyn Hei]) {
//     for h in s {
//         h.hei();
//     }
// }

// No one extend, because of Monomorphization
// pub fn add_true(v: &mut dyn Extend<bool>) {
//     v.extend(iter::once(true));
// }

pub fn it(v: &mut dyn Iterator<Item = bool>) {
    let _ = v.next();
}

// dyn Trait -> * -> (*mut data, *mut vtable)
// [u8]      -> * -> (*mut data, usize length)
// str       -> * -> (*mut data, usize length)

pub struct Wrapper<F>
where
    F: Fn(),
{
    _f: F,
    _g: Box<dyn Fn()>,
}

// Not dyn compatible
pub trait X {
    fn foo(&self, f: impl Fn());
}

// Is dyn compatible
pub trait Y {
    fn foo(&self, f: &dyn Fn());
}

// fn quox(x: &dyn X) {}
pub fn quoy(_y: &dyn Y) {}

/// trait object, both has vtable (pointer to calling function), and data pointe
///   r, can be called with closure
/// When you want to take trait object instead of generic, because otherwise you
///   have to propagate the generic type up -> clears up interface and nicer to
///   use
pub fn foo_dynfn(_f: &dyn Fn()) {}

/// Has to be function, not closure
/// Really a function pointer, single address pointer
pub fn bar_dynfn(_f: fn()) {}

/// can be called with closure, monomorphized to each individual closure
/// More generally usable, do not have to indirect behind pointer
/// Have to be cloned for each individual closure
pub fn baz_implfn(_f: impl Fn()) {}
