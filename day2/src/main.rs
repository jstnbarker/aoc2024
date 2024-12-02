use std::fs;
use std::env;

fn is_safe(line: &str) -> bool{
    let mut values: Vec<i32> = vec![];

    for substr in line.split(" "){
        values.push(substr.parse()
            .expect("Did not parse to int"));
    }
    println!("{line}");
    
    // need to determine wether inc or dec
    let mut inc: bool = false;
    if values[0] < values[1] {
        println!("Increasing");
        inc = true;
    } 
    else {
        println!("Decreasing");
    }

    for i in 0..values.len()-1{
        let a = values[i];
        let b = values[i+1];

        if inc && a >= b {
            return false;
        } else if !inc && a <= b {
            return false;
        }
        if (a-b).abs() > 3 {
            return false;
        }
    }
    println!("safe");
    return true;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(args[1].clone())
        .expect("Failed to read file");

    let mut safe_lines = 0;
    for line in input.lines(){
        if is_safe(line) {
            safe_lines += 1;
        }
        println!();
    }
    println!("Safe lines: {safe_lines}");
}
