extern crate libloading;
use std::collections::HashMap;
use std::mem::transmute;
use std::ffi::OsStr;

pub struct Library<'a> {
    lib: libloading::Library,
    fn_map: HashMap<&'a str, libloading::Symbol<'a, *const ()>>
}


impl<'a> Library<'a> {
    pub fn new<P: AsRef<OsStr>>(filename: P) -> std::io::Result<Library<'a>> {
        Ok(Library {
            lib: libloading::Library::new(filename)?,
            fn_map: HashMap::new()
        })
    }
    pub unsafe fn get<T>(&'a mut self, symbol: &'a str) -> std::option::Option<&'a libloading::Symbol<'a, T>> {
        match self.fn_map.get::<str>(symbol) {
            Some(func) => {
                let func = transmute::<&libloading::Symbol<*const()>, &libloading::Symbol<T>>(func);
                return Some(func);
            },
            None => {}
        };
        match self.lib.get(symbol.as_bytes()) {
            Ok(func) => {
                self.fn_map.insert(symbol, func);
                let func = self.fn_map.get::<str>(symbol).unwrap();
                let func = transmute::<&libloading::Symbol<*const()>, &libloading::Symbol<T>>(func);
                return Some(func);
            },
            Err(_) => return None
        };
    }
}
