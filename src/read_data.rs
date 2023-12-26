use std::fs::File;
use std::io::prelude::*;

pub fn read_data(fname: &str) -> String {
    let mut file = File::open(fname).unwrap();
    let mut s = String::new();

    file.read_to_string(&mut s);
    s
}