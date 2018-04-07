libfastdl-rust
=

## About
A libloading wrapper library that caches function pointer to hash table
## Usage
```rust
let mut lib = super::Library::new("libdltest.so").unwrap();
unsafe {
    let test_fn = lib.get::<unsafe extern fn(i32) -> i32>("test").unwrap();
    assert!(test_fn(100) == 200);
}
```
## License
Mozilla Public License 2.0
