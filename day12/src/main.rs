use std::fs;
fn load(path: String) -> Vec<Vec<char>> {
    let content = fs::read_to_string(path).expect("NO FILE");
    let mut out: Vec<Vec<char>> = Vec::new();
    for line in content.split('\n'){
        if !line.is_empty(){
            out.push(line.chars().collect());
        }
    }
    return out;
}

struct Plot{
    plot: Vec<Vec<char>>
} impl Plot {
    fn new() -> Self {
        Plot {
            plot:
        }
    }
    
    fn move(&self, dir:[usize;2]) -> Option<[usize;2]> {
        return Some(dir);
    }

    fn set(&mut self, coord: [usize;2], val: char){
        self.plot[coord[0]][coord[1]] = val;
    }

    fn get(&mut self, coord: [usize;2]) -> char{
        self.plot[coord[0]][coord[1]]
    }

    // return perimeter, area
    fn analyze(coord: [usize;2]) -> (u32, u32){
        // mark pos as visited

        // add one to area every step

        // check neighbors, add one to perimeter if neighbor is alphabetic and not the same char val as current
        // location

        // step to next neighbor if neighbor is same char as current
        
        // change visited mark to diff char
        return (0,0)
    }

}

use std::env::args;
fn main() {
    let args: Vec<String> = args().collect();
    let mut map = load(args[1].clone());

    for lat in 0..map.len(){
        for lon in 0..map[lat].len(){
            if map[lat][lon].is_alphabetic(){
            }
        }
    }
}
