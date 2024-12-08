use std::env;
use std::fs;

#[derive(Clone)]
struct Expr{
    result: u64,
    terms: Vec<u64>
}

fn load(path: String) -> Vec<Expr> {
    let content: String = fs::read_to_string(path)
        .expect("File not found");
    let content = content.split('\n');
    let mut expressions: Vec<Expr> = Vec::new();
    for line in content{
        if line.is_empty(){
            break;
        }
        let (result, values) = line.split_once(':').unwrap();
        let result: u64 = result.parse()
            .expect("Result did not parse");
        let mut terms: Vec<u64> = Vec::new();
        for value in values.split(' '){
            if !value.is_empty(){
                terms.push(value.parse()
                    .expect("Term did not parse"));
            }
        }
        expressions.push(Expr{result, terms})
    }
    return expressions;
}

fn solve(mut terms: Vec<u64>) -> [u64;2]{
    if terms.len() == 1 {
        let temp = terms.pop().expect("wait i htought there was one left");
        return [temp, temp]
    }
    /*
    for value in terms.clone() {
        print!("{} ", value);
    }
    println!();
    */
    let val = terms.pop().expect("aw shit i ran out of values");
    let result = solve(terms.clone());
    return [result[0]*val,result[1]+val];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let expr = load(args[1].clone());

    let mut sum: u64 = 0;
    for x in expr{
        for val in solve(x.terms.clone()){
            if val == x.result {
                sum += x.result;
            }
        }
    }
    println!("{}", sum);
}
