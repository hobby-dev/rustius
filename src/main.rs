//! # rustius
//! 
//! This is test project for a private company
//! 
#![deny(
    unused_imports,
    anonymous_parameters,
    box_pointers,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused,
    unsafe_code
)]
#![warn(variant_size_differences, missing_docs)]

use rustius::search_longest_2char_substr;

use std::io;
use std::io::prelude::*;

fn main() {
    println!("Enter string:");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("Longest substring of two characters:\n{}\nEnter string:", search_longest_2char_substr(&line.unwrap()));
    }
}
