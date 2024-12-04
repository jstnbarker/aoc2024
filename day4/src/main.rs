use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(args[1].clone()).expect("No file");

    let mut characters: Vec<Vec<char>> = Vec::new();
    for line in input.split("\n"){
        if !line.is_empty(){
            characters.push(line.chars().collect());
        }
    }
    for i in 0..characters.len(){
        for j in 0..characters[0].len(){
            let temp: char = characters[i][j];
            if temp == 'X'{
                println!("{}", temp);
            }
        }
    }
}
