use std::fs;
use std::env;

fn sum_distance(l: Vec<i32>, r: Vec<i32>) -> i32{
    let mut sum: i32 = 0;
    for i in 0..l.len(){
        sum += (r[i]-l[i]).abs();
    }
    return sum;
}

fn sum_similarity(l: Vec<i32>, r: Vec<i32>) -> i32{
    let mut sum = 0;
    for i in 0..l.len(){
        let target = l[i];
        let mut count = 0;
        for value in &r{
            if *value == target{
                count += 1;
            }
            else if *value > target{
                break;
            }
        }
        sum += count * target;
    }
    return sum;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(args[1].clone())
        .expect("Should have read the file");

    let mut l:Vec<i32> = Vec::new();
    let mut r:Vec<i32> = Vec::new();

    for line in contents.lines(){
        let values:Vec<&str> = line.split("   ").collect();
        l.push(values[0].parse().unwrap());
        r.push(values[1].parse().unwrap());
    }
    l.sort();
    r.sort();
    
    let dist = sum_distance(l.clone(), r.clone());
    println!("Total distance: {dist}");
    let sim = sum_similarity(l.clone(), r.clone());
    println!("Total similarity: {sim}");
}
