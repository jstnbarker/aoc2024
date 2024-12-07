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

#[derive(PartialEq)]
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

    pub fn clone(&self) -> Self {
        Guard {
            pos: self.pos.clone(),
            axis_limit: self.axis_limit,
            dir: self.dir.clone(),
            movement_vector: self.movement_vector
        }
    }

    pub fn turn(&mut self, times: usize){
        self.dir = (self.dir + times) % 4
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
        self.pos = temp;
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
    
    let mut map: Map = Map::new(map);
    let mut guard = Guard::new(guard_pos, map.limits);
    let mut unique_tiles = 1; // otherwise doesn't count start positio  
    let mut loops = 0;
    loop{
        if map.get(guard.next()) == '#'{ 
            guard.turn(1)
        } else {
            let tile = map.get(guard.pos);
            if tile == '.'{
                map.set(guard.pos, 'x');
                unique_tiles += 1;
            } 
            println!("{}", map);

            let mut future = guard.clone();
            if guard.next() != guard.pos{
                let orig = map.get(guard.next()); // save next tile's value
                map.set(guard.next(),'#');        // set next tile to '#'
                println!("{}", map);              
                future.turn(1);                   // turn ghost away from '#'
                let max_steps = 10000;
                for i in 0..max_steps{
                    let future_tile = map.get(future.next());
                    if future_tile == '#'{
                        future.turn(1);
                    } else if future == guard || i == max_steps-1 {
                        loops += 1;
                        break;
                    }
                    if !future.step(){
                        break;
                    }
                }
                map.set(guard.next(),orig);      // reset next tile to original value
            } else {
                break;
            }
            guard.step();
        }

    }
    println!("{}\n{}",unique_tiles,loops);
}
/*
// 1614 too low
// 1706 wrong
// 1808 too high
// 1853 too high
// (untested) 1862 too high
//
// 10,000,000 1708
//  1,000,000 1676
//    100,000 1707
//     10,000 1706
//      1,000 2569
*/
