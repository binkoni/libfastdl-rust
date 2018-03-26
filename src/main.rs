extern crate libloading as lib;
extern crate fastdl;
fn main() {
    let mut lib = fastdl::Library::new("libdylibexample.so").unwrap();
    unsafe {
        match lib.get::<unsafe extern fn()>("test") {
            Some(test_fn) => test_fn(),
            None => println!("NONE!")
        }
    }
}
