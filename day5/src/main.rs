use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(args[1].clone())
        .expect("File not found");
    let mut line_iter = content.split("\n");
    
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    loop{
        let current = line_iter.next().unwrap();
        if !current.is_empty(){
            let values: Vec<&str> = current.split("|").collect();
            let l:i32 = values[0].parse().unwrap();
            let r:i32 = values[1].parse().unwrap();

            let temp = rules.get_mut(&l);
            if temp.is_some(){
                temp.unwrap().push(r);
            } else {
                rules.insert(l, vec![r]);
            }
        } else {
            break;
        }
    }

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
    let mut corrected_sum = 0;
    for update in updates{
        if check_update(update.clone(), rules.clone()).is_none() {
            sum += update[update.len()/2];
        } else {
            corrected_sum += correct_update(update.clone(),rules.clone())[update.len()/2];
        }
    }
    println!("{}\n{}", sum, corrected_sum );
}

fn check_update(update: Vec<i32>, rules: HashMap<i32, Vec<i32>>) -> Option<[usize; 2]>{
    for page in 0..update.len(){
        let applicable_rules = rules.get(&update[page]);
        if applicable_rules.is_some(){
            for rule in applicable_rules.unwrap(){
                for other_page in 0..update.len(){
                    if update[other_page] == *rule{
                        if other_page < page{
                            // return failing indices
                            return Some([page,other_page]);
                        }
                    }
                }
            }
        }
    }
    return None;
}

fn correct_update(mut bad_update: Vec<i32>, rules: HashMap<i32, Vec<i32>>) -> Vec<i32>{
    let temp = check_update(bad_update.clone(), rules.clone());
    if temp.is_some(){
        let temp = temp.unwrap();
        bad_update.swap(temp[0], temp[1]);
        return correct_update(bad_update, rules.clone());
    }
    return bad_update;
}
