#![cfg(test)]

use libloading;
#[test]
fn success() {
    let mut lib = super::Library::new("libdltest.so").unwrap();
    unsafe {
        let test_fn = lib.get::<unsafe extern fn(i32) -> i32>("test").unwrap();
        assert!(test_fn(100) == 200);
    }
}

#[test]
#[should_panic]
fn file_not_found() {
    let mut lib = super::Library::new("libwhatever.so").unwrap();
    unsafe {
        let test_fn = lib.get::<unsafe extern fn(i32) -> i32>("test").unwrap();
        assert!(test_fn(100) == 200);
    }
}

#[test]
#[should_panic]
fn symbol_not_found() {
    let mut lib = super::Library::new("libdltest.so").unwrap();
    unsafe {
        let test_fn = lib.get::<unsafe extern fn(i32) -> i32>("whatever").unwrap();
        assert!(test_fn(100) == 200);
    }
}

#[test]
fn libloading() {
    let lib = libloading::Library::new("libdltest.so").unwrap();
    unsafe {
        let test_fn: libloading::Symbol<unsafe extern fn(i32) -> i32> = lib.get(b"test").unwrap();
        assert!(test_fn(100) == 200);
    }
}
