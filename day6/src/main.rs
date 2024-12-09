use std::fs;
use std::env;
use std::fmt;

struct Map{
    map: Vec<Vec<char>>,
    limits: [usize; 2]
}

impl Map{
    pub fn new(map: Vec<Vec<char>>) -> Self {
        return Map{
            limits: [map.len(), map[0].len()],
            map
        }
    }

    pub fn get(&self, coord: [usize;2])-> char{
        return self.map[coord[0]][coord[1]].clone()
    }
    pub fn set(&mut self, coord: [usize;2], val: char){
        self.map[coord[0]][coord[1]] = val;
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = "".to_string();

        for y in &self.map{
            for x in y{
                result = result + &x.to_string();
            }
            result+="\n";
        }
        return write!(f, "{} ", result)
    }
}

struct Guard{
    pub pos: [usize;2],
    pub dir:  usize,
    axis_limit: [usize;2],
    movement_vector: [[i32; 2]; 4],
    pedometer: usize
}

impl PartialEq for Guard{
    fn eq(&self, other: &Self) -> bool {
        let a = self.dir == other.dir;
        let b = self.pos == other.pos;
        return a&b;
    }
}

impl Guard{
    pub fn new(pos: [usize;2], axis_limit: [usize;2]) -> Self {
        Guard {
            pos,
            axis_limit,
            dir: 0,
            movement_vector: [[-1,0],[0,1],[1,0],[0,-1]],
            pedometer: 0
        }
    }

    pub fn clone(&self) -> Self {
        Guard {
            pos: self.pos.clone(),
            axis_limit: self.axis_limit,
            dir: self.dir.clone(),
            movement_vector: self.movement_vector,
            pedometer: 0,
        }
    }

    pub fn turn(&mut self){
        self.dir = (self.dir + 1) % 4
    }

    pub fn next(&mut self) -> [usize;2] {
        let ni = self.movement_vector[self.dir][0] + self.pos[0] as i32;
        let nj = self.movement_vector[self.dir][1] + self.pos[1] as i32;

        if ni < 0 || 
           ni >= self.axis_limit[0] as i32 ||
           nj < 0 || 
           nj >= self.axis_limit[1] as i32 
        {
            return self.pos;
        }
        return [ni as usize, nj as usize];
    }

    pub fn step(&mut self) -> bool{
        let temp = self.next();
        if self.pos == temp{
            return false;
        }
        self.pedometer += 1;
        self.pos = temp;
        return true;
    }
}
impl fmt::Display for Guard{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({}, {}): {} ", self.pos[0], self.pos[1], self.dir)
    }
}

const STEP_MAX: usize = 5000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(args[1].clone())
        .expect("Could not read da file");

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut guard_pos: [usize;2] = [0,0];
    let mut found: bool = false;
    let content: Vec<&str> = content.split('\n').collect();
    for line in 0..content.len()-1{
        if content[line].is_empty(){
            break;
        }
        if !found{
            let depth = content[line].find('^');
            if depth.is_some(){
                guard_pos = [line,depth.unwrap()];
                found = true;
            }
        }
        grid.push(content[line].chars().collect());
    }
    
    /* --- part 1 --- */
    let mut map: Map = Map::new(grid.clone());
    let mut guard = Guard::new(guard_pos, map.limits);
    let mut unique_tiles = 1; // otherwise doesn't count start positio  
    let mut unique_positions = 0;
    loop{
        if map.get(guard.next()) == '#'{ 
            guard.turn()
        } 
        if map.get(guard.pos) == '.'{
            map.set(guard.pos, 'x');
            unique_tiles += 1;
        }
        let tile = map.get(guard.next()).clone();
        if tile == '.'{
            map.set(guard.next(), '#');
            let mut ghost = guard.clone();
            ghost.turn();
            loop {
                // there are  no washing machines in main input
                while map.get(ghost.next()) == '#' {
                    ghost.turn();
                }
                /*
                let g_temp = map.get(ghost.pos);
                let temp = map.get(guard.pos);
                map.set(ghost.pos, 'G');
                map.set(guard.pos, '$');
                println!("{}\n{}", map,ghost.pedometer);
                map.set(ghost.pos, g_temp);
                map.set(guard.pos, temp);
                */
                
                if !ghost.step(){
                    map.set(guard.next(), tile);
                    break;
                }

                if  ghost == guard || ghost.pedometer > STEP_MAX {
                    map.set(guard.next(), '&');
                    break;
                }


            }
        }
        if !guard.step(){
            break;
        }
    }
    println!("Part 1: {}",unique_tiles);

    /* --- part 2 --- */
    // restart with fresh objects
    for i in map.map{
        for j in i{
            if j == '&' {
                unique_positions += 1;
            }
        }
    }
    println!("Part 2: {}",unique_positions);
}
/*
1589
1614 too low
1706 wrong
1746 wrong
1758 wrong; figured my ghost might get cornered 
tryin' 1759 after an adjustment  **WRONG**
1808 too high
1824
1827
1848
1853 too high
(untested) 1862 too high
*/
