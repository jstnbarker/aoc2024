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

use std::fmt;
struct Plot{
    plot: Vec<Vec<char>>,
    directions: [[i32;2];4],
    limit: [i32;2],
} impl Plot {
    fn new(plot: Vec<Vec<char>>) -> Self {
        Plot {
            limit: [plot.len() as i32, plot[0].len() as i32],
            plot,
            directions: [[1,0],[0,1],[-1,0],[0,-1]],
        }
    }
    
    // return None if out of bounds, otherwise return Some coord after applying step direction
    fn step(&self, coord: [usize;2], dir:[i32;2]) -> Option<[usize;2]> {
        let new_lat = dir[0] + coord[0] as i32;
        let new_lon = dir[1] + coord[1] as i32;
        if new_lat >= self.limit[0] || new_lat < 0 || 
           new_lon >= self.limit[1] || new_lon < 0 {
            return None;
        }
        return Some([new_lat as usize, new_lon as usize])
    }

    fn set(&mut self, coord: [usize;2], val: char){
        self.plot[coord[0]][coord[1]] = val;
    }

    fn get(&mut self, coord: [usize;2]) -> char{
        self.plot[coord[0]][coord[1]]
    }

    // circle clockwise 
    // false, true/false, false == outside corner
    // true, false, true == inside corner
    fn count_corners(&mut self, coord:[usize;2]) -> u32 {
        let mut corners = 0;
        let mut neighbors: [bool;8] = [false;8];
        let directions = [[-1,0],[-1,1],[0,1],[1,1],[1,0],[1,-1],[0,-1],[-1,-1]];
        let target = self.get(coord);
        for i in 0..directions.len(){
            match self.step(coord, directions[i]){
                Some(neighbor) => {
                    let nval = self.get(neighbor);
                    neighbors[i] = nval == target || nval == target.to_ascii_lowercase();
                }
                None => {}
            }
        }

        let mut start = 0;
        for _ in 0..4{
            let a = neighbors[start];
            let b = neighbors[start+1];
            let c = neighbors[(start+2)%8];

            if a {
                if !c {
                    if b {
                        corners+=1
                    }
                }
            } else {
                if !c{
                    corners+=1;
                }
            }
            start += 2;
        }
        return corners;
    }

    // paint bucket fill
    fn fill(&mut self, coord:[usize;2],val:char){
        self.set(coord, val);
        for dir in self.directions{
            match self.step(coord, dir){
                Some(neighbor) => {
                    if '.' == self.get(neighbor){
                        self.fill(neighbor, val);
                    }
                }
                None => {}
            }
        }
    }
    // return perimeter, area
    fn analyze(&mut self, coord: [usize;2]) -> (u32, u32, u32){
        let original = self.get(coord);

        // add one to area every step
        let mut area = 1;
        let mut perimeter = 0;
        let mut corners = self.count_corners(coord);

        // mark pos as visited
        let lower = original.to_ascii_lowercase();
        self.set(coord, lower);

        // check neighbors, add one to perimeter if neighbor is alphabetic and not the same char val as current
        // location
        for dir in self.directions{
            match self.step(coord, dir){
                Some(neighbor) => {
                    let nval = self.get(neighbor);
                    if nval != original && nval != lower {
                        perimeter += 1;
                    } else if nval == original {
                        // step to next neighbor if neighbor is same char as current
                        let (p, a, c) = self.analyze(neighbor);
                        perimeter += p;
                        area += a;
                        corners += c;
                    }
                }
                None => {
                    perimeter += 1;
                }
            }
        }
        (perimeter, area, corners)
    }
}
impl fmt::Display for Plot{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "".to_string();
        for lat in self.plot.clone(){
            for char in lat {
                out.push(char);
            }
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}

use std::env::args;
fn main() {
    let args: Vec<String> = args().collect();
    let mut plot = Plot::new(load(args[1].clone()));

    let mut sum: u32 = 0;
    let mut p2sum: u32 = 0;
    for lat in 0..plot.plot.len(){
        for lon in 0..plot.plot[0].len(){
            let coord = [lat,lon];
            if plot.get(coord).is_uppercase(){
                let (perim, area, corners) = plot.analyze(coord);
                let region_cost = perim * area;
                sum += region_cost;
                p2sum += area*corners;
                //plot.fill(coord,'#');
                //println!("{}", plot);
            }
        }
    }
    println!("Fence cost: {}", sum);
    println!("With bulk discount: {}", p2sum);
}
