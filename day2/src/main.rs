use std::fs;
use std::env;

fn is_safe(report: Vec<i32> ) -> bool{

    // need to determine wether inc or dec
    let mut inc: bool = false;
    if report[0] < report[1] {
        inc = true;
    } 

    for i in 0..report.len()-1{
        let a = report[i];
        let b = report[i+1];

        if inc && a >= b {
            return false;
        } else if !inc && a <= b {
            return false;
        }
        if (a-b).abs() > 3 {
            return false;
        }
    }
    return true;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(args[1].clone())
        .expect("Failed to read file");

    let mut safe_lines = 0;
    for line in input.lines(){
        let mut values: Vec<i32> = vec![];
        for substr in line.split(" "){
            values.push(substr.parse()
                .expect("Did not parse to int"));
        }
        if is_safe(values) {
            safe_lines += 1;
        }
    }
    println!("Safe lines: {safe_lines}");
}
