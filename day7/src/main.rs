use std::env;
use std::fs;

struct Expr{
    result: u64,
    terms: Vec<i32>
}

fn load(path: String) -> Vec<Expr> {
    let content: String = fs::read_to_string(path)
        .expect("File not found");
    let content = content.split('\n');
    let mut expressions: Vec<Expr> = Vec::new();
    for line in content{
        let (result, values) = line.split_once(':').unwrap();
        let result: u64 = result.parse()
            .expect("Result did not parse");
        let mut terms: Vec<i32> = Vec::new();
        for value in values.split(' '){
            terms.push(value.parse()
                .expect("Term did not parse"));
        }
        expressions.push(Expr{result, terms})
    }
    return expressions;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let expr = load(args[1].clone());
}
