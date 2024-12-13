use nalgebra;
use std::env;
use std::fs;
use regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(args[1].clone()).expect("NO FILE");

    let mut groups = content.split("\n\n");
    loop {
        match groups.next() {
            Some(group) => {
            }
            None => {}
        }
    }
}
