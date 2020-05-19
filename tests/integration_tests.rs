//! integration_tests.rs
//!
//! test all the functions

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/dog.rs"));

use std::os::raw::c_char;

#[test]
fn test_dog() {
    unsafe {
        let name: *const c_char = "Bob".as_ptr() as *const c_char;
        let mut dog = Dog::new(name);
        dog.walk();
        dog.stop();
    }
}
