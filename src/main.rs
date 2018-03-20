extern crate libloading as lib;
use std::collections::HashMap;
use std::mem::transmute;
fn main() {
    let lib = lib::Library::new("libdylibexample.so").unwrap();
    let mut fn_map: HashMap<&str, lib::Symbol<*const ()>> = HashMap::new();
    unsafe {
        let func: lib::Symbol<*const ()> = lib.get(b"test").unwrap();
        fn_map.insert("test", func);
        let func = fn_map.get("test").unwrap();
        let func = transmute::<&lib::Symbol<*const()>, &lib::Symbol<unsafe extern fn()>>(func);
        func();
    }
    println!("This is main function");
}
