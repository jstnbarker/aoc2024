use std::fs;
use std::env;

fn load(path: String) -> Vec<Vec<u32>> {
    let mut out: Vec<Vec<u32>> = Vec::new();
    let content = fs::read_to_string(path).expect("No file");
    let mut input = content.chars();
    let mut builder: Vec<u32> = Vec::new();
    loop{
        match input.next(){
            Some(char) => {
                if char == '\n'{
                    if !builder.is_empty(){
                        out.push(builder);
                    }
                    builder = Vec::new();
                } else {
                    builder.push(char.to_digit(10).unwrap())
                }
            }
            None => break
        }
    }
    return out;
}

struct PathFinder{
    topo: Vec<Vec<u32>>,
    lim: [usize;2],
    offsets: [[i32;2];4],
}

impl PathFinder{
    fn new(topo: Vec<Vec<u32>>) -> Self {
        return PathFinder{
            lim: [topo.len(),topo[0].len()],
            topo,
            offsets: [[1,0],[0,-1],[-1,0],[0,1]]
        }
    }

    fn apply_offset(&self, coord:[usize;2], dir:[i32;2]) -> Option<[usize;2]>{
        let i = coord[0] as i32 + dir[0];
        let j = coord[1] as i32 + dir[1];
        if j < 0 || i < 0 || j >= self.lim[1] as i32 || i >= self.lim[1] as i32{
            return None
        }
        return Some([i as usize, j as usize]);
    }

    fn solve(&mut self) -> i32 {
        let mut sum_of_scores = 0;
        for i in 0..self.lim[0]{
            for j in 0..self.lim[1]{
                if self.topo[i][j] == 0 {
                    sum_of_scores += self.find_trail([i,j]);
                }
            }
        }
        sum_of_scores
    }

    fn at(&self, coord:[usize;2]) -> u32 {
        self.topo[coord[0]][coord[1]]
    }
    fn set(&mut self, coord:[usize;2], value: u32) {
        self.topo[coord[0]][coord[1]] = value;
    }
    
    fn find_trail(&mut self, coord:[usize;2]) -> i32 {
        if self.at(coord) == 9{
            //self.set(coord, 9);
            return 1;
        }
        let mut sum = 0;
        for offset in self.offsets{
            match self.apply_offset(coord,offset){
                Some(result) => {
                    if self.at(result) == self.at(coord)+1{
                        sum += self.find_trail(result);
                    }
                }
                None => {}
            }
        }
        return sum;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut x = PathFinder::new(load(args[1].clone()));
    println!("{}", x.solve());
}
