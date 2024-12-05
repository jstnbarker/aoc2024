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
    let mut total_cross_mas: i32 = 0;
    for i in 0..characters.len(){
        for j in 0..characters[0].len(){
            let temp: char = characters[i][j];
            if temp == 'X'{
                total_xmas += find_xword(i,j,characters.clone(),"XMAS".to_string());
            }
            if temp == 'A'{
                if validate_cross_mas(i,j,characters.clone()){
                    total_cross_mas += 1;
                }
            }
        }
    }
    println!("{}", total_xmas);
    println!("{}", total_cross_mas);
}

fn find_xword(i:usize, j: usize, characters: Vec<Vec<char>>, pattern: String) -> i32{
    let offsets: [[i32; 2]; 8] = [[-1,-1],[-1,0],[-1,1],[0,-1],[0,1],[1,-1],[1,0],[1,1]];
    let mut sum: i32 = 0;
    for offset in offsets{
        let mut pos = [i,j];
        let mut builder: String = characters[pos[0]][pos[1]].to_string();
        while pattern.contains(&builder) {
            let check = add(pos, offset);
            if check .is_none(){
                break;
            } else {
                pos = check.unwrap();
            }
            // ensure current position is not beyond vector limits
            if pos[0] >= characters.len() || pos[1] >= characters[0].len(){
                break;
            }

            builder.push(characters[pos[0]][pos[1]]);

            if builder == pattern{
                sum += 1;
                break;
            }
        }
    }
    return sum;
}

fn validate_cross_mas(i: usize, j:usize, characters: Vec<Vec<char>>) -> bool {
    let pattern = [[-1,-1],[1,1],[-1,1],[1,-1],];
    let mut builder: String = "".to_string();
    let mut occurances: i32 = 0;
    for k in 0..pattern.len(){
        let mut pos = [i,j];
        let check = add(pos, pattern[k]);
        if check.is_none(){
            return false;
        }
        pos = check.unwrap();
        // ensure current position is not beyond vector limits
        if pos[0] >= characters.len() || pos[1] >= characters[0].len(){
            return false;
        }
        let current = characters[pos[0]][pos[1]];
        builder.push(current);
        let current_length = builder.len();
        if current_length == 2{
            if builder == "MS"{
                occurances += 1;
            } else if builder == "SM"{
                occurances += 1;
            }
            builder = "".to_string();
        }
    }
    if occurances == 2 {
        return true
    }
    return false
}
// 1784 wrong

fn add(mut coordinate: [usize; 2], offset: [i32;2]) -> Option<[usize; 2]> {
    let ni = coordinate[0] as i32 + offset[0];
    let nj = coordinate[1] as i32 + offset[1];
    if ni < 0 || nj < 0 {
        return Option::None;
    } else {
        coordinate[0] = ni as usize;
        coordinate[1] = nj as usize;
    }
    return Some(coordinate);
}
