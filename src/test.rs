#[cfg(test)]
#[test]
fn test_success() {
    let mut lib = super::Library::new("libdylibexample.so").unwrap();
    unsafe {
        let test_fn = lib.get::<unsafe extern fn(i32) -> i32>("test").unwrap();
        assert!(test_fn(100) == 200);
    }
}

#[test]
#[should_panic]
fn test_file_not_found() {
    let mut lib = super::Library::new("libwhatever.so").unwrap();
    unsafe {
        let test_fn = lib.get::<unsafe extern fn(i32) -> i32>("test").unwrap();
        assert!(test_fn(100) == 200);
    }
}

#[test]
#[should_panic]
fn test_symbol_not_found() {
    let mut lib = super::Library::new("libdylibexample.so").unwrap();
    unsafe {
        let test_fn = lib.get::<unsafe extern fn(i32) -> i32>("whatever").unwrap();
        assert!(test_fn(100) == 200);
    }
}
