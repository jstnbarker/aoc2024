use std::env;
use std::fs;
use regex;

fn multiply(expression: &str) -> i32 {
    let values: Vec<&str> = expression[0..expression.len()-1][4..].split(',').collect();
    let a: i32 = values[0].parse()
        .expect("");
    let b: i32 = values[1].parse()
        .expect("");
    return a*b;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(args[1].clone()).unwrap();
    let pattern = regex::Regex::new(r"mul\(\d*,\d*\)").unwrap();
    let mut index = 0;
    let mut sum = 0;
    loop {
        let current = pattern.find_at(&content,index);
        if current.is_none(){
            break;
        }
        index = current.unwrap().end();
        let expr = current.unwrap().as_str();
        sum += multiply(expr);
    }
    println!("{}",sum);
}
