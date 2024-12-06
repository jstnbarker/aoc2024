use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(args[1].clone())
        .expect("Could not read da file");

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in content.split('\n'){
        if !line.is_empty(){
            map.push(line.chars().collect());
        }
    }


    // find guard
    let mut hash_locations: Vec<[usize; 2]> = Vec::new();
    let mut guard_pos = [0,0];
    for i in 0..map.len(){
        for j in 0..map[0].len(){
            let x = map[i][j];
            if x == '#'{
                hash_locations.push([i,j]);
            } else if x == '^'{
                guard_pos = [i,j];
            }
        }
    }

    let movement_vectors = [[-1,0],[0,1],[1,0],[0,-1]];
    let crumb = ['^','>','v','<'];
    let mut direction = 0;
    let mut unique_tiles = 1;
    let mut potential_loops = 0;
    loop{
        let tile = &mut map[guard_pos[0]][guard_pos[1]];
        if *tile == '#'{
            let temp = apply_move(guard_pos, movement_vectors[(direction+2)%4]);
            if temp.is_none(){
                break;
            }
            guard_pos = temp.unwrap();
            direction = (direction + 1) % 4;
        } else if *tile == '.'{
            *tile = crumb[direction];
            unique_tiles += 1;
        } else {
            if *tile == crumb[(direction+1)%4]{
                println!("POTENTIAL LOOP AT {}, {}", guard_pos[0], guard_pos[1]);
                potential_loops+=1;
            }
        }
        guard_pos = apply_move(guard_pos, movement_vectors[direction]).unwrap();
        if guard_pos[0] >= map.len() || guard_pos[1] >= map[0].len(){
            break;
        }
        print_map(map.clone());
    }
    println!("{}", unique_tiles);
    println!("{}", potential_loops);
}
fn print_map(map: Vec<Vec<char>>){
    for y in map{
        for x in y {
            print!("{}", x);
        }
        println!();
    }
}

fn apply_move(mut pos:[usize;2], movement_vector: [i32;2]) -> Option<[usize;2]> {
    let ni = movement_vector[0] + pos[0] as i32;
    let nj = movement_vector[1] + pos[1] as i32;

    if ni < 0 || nj < 0 {
        return None;
    }

    pos[0] = ni as usize;
    pos[1] = nj as usize;
    return Some(pos);
}
