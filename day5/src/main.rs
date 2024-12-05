use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(args[1].clone())
        .expect("File not found");
    let mut line_iter = content.split("\n");
    
    // load ordering rules 
    let mut rules: Vec<[i32; 2]> = Vec::new();
    loop{
        let current = line_iter.next().unwrap();
        if current.is_empty(){
            break;
        } else {
            let values: Vec<&str> = current.split("|").collect();
            rules.push([
                values[0].parse().unwrap(),
                values[1].parse().unwrap()
            ]);
        }
    }

    // load updates
    let mut updates: Vec<Vec<i32>> = Vec::new();
    loop{
        let current = line_iter.next().unwrap();
        if current.is_empty(){
            break;
        } else {
            let mut upd_iter = current.split(",");
            let mut update: Vec<i32> = Vec::new();
            loop{
                let current = upd_iter.next();
                if current.is_none(){
                    break;
                }
                update.push(current.unwrap().parse().expect("Did not parse to i32"))
            }
            updates.push(update);
        }
    }
}
