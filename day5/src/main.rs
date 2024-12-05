use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(args[1].clone())
        .expect("File not found");
    let mut line_iter = content.split("\n");
    
    // load ordering rules 
    // O(n) runtime 
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
    // O(nm) runtime
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
                update.push(current.unwrap().parse()
                    .expect("Could not parse to i32"))
            }
            updates.push(update);
        }
    }

    let mut sum = 0;
    for update in updates{
        if check_update(update.clone(), rules.clone()) {
            sum += update[update.len()/2];
            for value in update{
                print!("{} ", value);
            }
            println!();
        }
    }
    println!("{}", sum);
}
/*
fn get_applicable_rules(update: Vec<i32>, rules: Vec<[i32;2]>) -> Vec<[i32; 2]>{
    let mut to_apply: Vec<[i32;2]> = Vec::new();
    for page in 0..update.len(){
        for rule in rules
    }
    return to_apply;
}
*/


fn check_update(update: Vec<i32>, rules: Vec<[i32; 2]>) -> bool {
    for page in 0..update.len(){
        for rule in &rules{
            // only check applicable rule
            if rule[0] == update[page]{
                // then check remainder of list for rule[1]
                for other_page in 0..update.len(){
                    let temp = update[other_page]; 
                    if rule[1] == temp {
                        if other_page < page{
                            return false;
                        }
                    }
                }
            }
        }
    }
    return true;
}
