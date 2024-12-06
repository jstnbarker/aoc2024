use std::fs;
use std::env;

struct Guard{
    pub pos: [usize;2],
    pub dir:  usize,
    axis_limit: [usize;2],
    movement_vector: [[i32; 2]; 4],
}

impl Guard{
    pub fn new(pos: [usize;2], axis_limit: [usize;2]) -> Self {
        Guard {
            pos,
            axis_limit,
            dir: 0,
            movement_vector: [[-1,0],[0,1],[1,0],[0,-1]],
        }
    }

    pub fn turn(&mut self, times: usize){
        self.dir = (self.dir + times) % 4
    }

    pub fn step(&mut self) -> bool{
        let ni = self.movement_vector[self.dir][0] + self.pos[0] as i32;
        let nj = self.movement_vector[self.dir][1] + self.pos[1] as i32;

        if ni < 0 || 
           ni >= self.axis_limit[0] as i32 ||
           nj < 0 || 
           nj >= self.axis_limit[1] as i32 
        {
            return false;
        }
        self.pos = [ni as usize, nj as usize];
        return true;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(args[1].clone())
        .expect("Could not read da file");

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut guard_pos: [usize;2] = [0,0];
    let mut found: bool = false;
    let content: Vec<&str> = content.split('\n').collect();
    for line in 0..content.len()-1{
        if !found{
            let depth = content[line].find('^');
            if depth.is_some(){
                guard_pos = [line,depth.unwrap()];
                found = true;
            }
        }
        map.push(content[line].chars().collect());
    }

    let mut guard = Guard::new(guard_pos, [map.len(),map[0].len()]);
    let crumbs = ['^','>','v','<'];
    let mut unique_tiles = 1; // otherwise doesn't count start position
    loop{
        let tile = &mut map[guard.pos[0]][guard.pos[1]];
        if *tile == '#'{
            guard.turn(2);
            guard.step();
            guard.turn(3);
        } else if *tile == '.'{
            *tile = crumbs[guard.dir];
            unique_tiles += 1;
        }
        if !guard.step(){
            break;
        }
    }
    println!("{}", unique_tiles);
}
