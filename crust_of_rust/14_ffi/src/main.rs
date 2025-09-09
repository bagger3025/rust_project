// mod foo {
//     include!(concat!(env!("OUT_DIR"), "/foo.rs"));
// }

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    println!("{}", env!("OUT_DIR"));

    // foo::foo();
}
