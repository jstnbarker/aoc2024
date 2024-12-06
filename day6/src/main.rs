use std::fs;
use std::env;

struct Guard{
    pos: [usize;2],
    dir:  usize,
}

impl Guard{
    pub fn new(pos: [usize;2]) -> Self {
        Guard {
            pos,
            dir: 0,
        }
    }

    pub fn turn(&mut self){
        self.dir = (self.dir + 1) % 4
    }

    pub fn step_back(&mut self) -> bool{
        self.dir = (self.dir + 2) % 4;
        self.step();
        self.dir = (self.dir + 2) % 4;
        return true;
    }
    pub fn get_dir(&self) -> usize {
        return self.dir;
    }

    pub fn get_pos(&self) -> [usize; 2]{
        return self.pos;
    }

    pub fn step(&mut self) -> bool{
        let movement_vector: [[i32; 2]; 4] = [[-1,0],[0,1],[1,0],[0,-1]];
        let ni = movement_vector[self.dir][0] + self.pos[0] as i32;
        let nj = movement_vector[self.dir][1] + self.pos[1] as i32;

        if ni < 0 || nj < 0 {
            return false;
        }

        self.pos[0] = ni as usize;
        self.pos[1] = nj as usize;
        return true;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(args[1].clone())
        .expect("Could not read da file");

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut guard_pos: [usize;2] = [0,0];
    let content: Vec<&str> = content.split('\n').collect();
    for line in 0..content.len(){
        if !content[line].is_empty(){
            let depth = content[line].find('^');
            if depth.is_some(){
                guard_pos = [line,depth.unwrap()];
            }
            map.push(content[line].chars().collect());
        }
    }

    let mut guard = Guard::new(guard_pos);
    let crumb = ['^','>','v','<'];
    let mut unique_tiles = 1;
    loop{
        guard_pos = guard.get_pos();
        let guard_dir = guard.get_dir();
        let tile = &mut map[guard_pos[0]][guard_pos[1]];
        if *tile == '#'{
            guard.step_back();
            guard.turn();
        } else if *tile == '.'{
            *tile = crumb[guard_dir];
            unique_tiles += 1;
        }
        if guard_pos[0] >= map.len() || guard_pos[1] >= map[0].len(){
            break;
        }
        guard.step();
        print_map(map.clone());
    }
    println!("{}", unique_tiles);
}

fn print_map(map: Vec<Vec<char>>){
    for y in map{
        for x in y {
            print!("{}", x);
        }
        println!();
    }
}

