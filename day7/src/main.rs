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
        terms.reverse();
        expressions.push(Expr{result, terms})
    }
    return expressions;
}

fn concat(a:u64, b:u64) -> u64 {
    let result = (a.to_string() + &b.to_string()).parse().expect("big fuckin' number");
    //println!("{}", result);
    return result;
}

fn solve(current:u64, target: u64, mut terms: Vec<u64>) -> u64{
    let val = terms.pop().unwrap();
    let a = current * val;
    let b = current + val;
    let c = concat(current,val);
    if terms.is_empty(){
        if a == target{
            return a;
        } 
        else if b == target{
            return b;
        } 
        else if c == target{
            return c;
        }
        return 0;
    } 
    if solve(a,target,terms.clone()) == target{
        return target;
    } 
    else if solve(b,target,terms.clone()) == target {
        return target;
    } 
    else if solve(c,target,terms.clone()) == target{
        return solve(c,target,terms.clone());
    }
    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let expr = load(args[1].clone());

    let mut sum: u64 = 0;
    for mut x in expr{
        if solve(x.terms.pop().unwrap(), x.result, x.terms) == x.result{
            sum += x.result;
        }
    }
    println!("{}", sum);
}
