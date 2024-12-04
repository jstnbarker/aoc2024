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
    let mut total_xmas: i32 = 0;
    for i in 0..characters.len(){
        for j in 0..characters[0].len(){
            let temp: char = characters[i][j];
            if temp == 'X'{
                total_xmas += find_xmas(i,j,characters.clone());
            }
        }
    }
    println!("{}", total_xmas);
}

fn find_xmas(i:usize, j: usize, characters: Vec<Vec<char>>) -> i32{
    let offsets: [[i32; 2]; 8] = [[-1,-1],[-1,0],[-1,1],[0,-1],[0,1],[1,-1],[1,0],[1,1]];
    let pattern = "XMAS";
    let mut sum: i32 = 0;
    for offset in offsets{
        let mut pos = [i,j];
        let mut temp: String = "X".to_string();
        while pattern.contains(&temp) {
            // apply offset
            let ni = pos[0] as i32 + offset[0];
            let nj = pos[1] as i32 + offset[1];

            // make sure applied offset is within boundaries
            if ni >= 0 && ni < characters.len() as i32 {
                pos[0] = ni as usize;
            } else {
                break;
            }
            if nj >= 0 && nj < characters[0].len() as i32{
                pos[1] = nj as usize;
            } else {
                break;
            }

            // check character value at new pos
            temp.push(characters[pos[0]][pos[1]]);
            println!("{}", temp);

            if temp == pattern{
                sum += 1;
                break;
            }
        }
    }
    return sum;
}
