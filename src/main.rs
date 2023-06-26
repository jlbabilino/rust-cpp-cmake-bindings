// Use all the autocxx types which might be handy.
use autocxx::prelude::*;

include_cpp! {
    #include "mylibrary.h"
    safety!(unsafe_ffi)
    generate!("my_special_function") // allowlist a function
}

fn main() {
    assert_eq!(ffi::my_special_function(12.0), -12.0);
    println!("{}", ffi::my_special_function(13.0));
}