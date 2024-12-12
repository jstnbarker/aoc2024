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
    fn analyze(&mut self, coord: [usize;2]) -> (u32, u32){
        let original = self.get(coord);

        // mark pos as visited
        self.set(coord, '.');

        // add one to area every step
        let mut area = 1;
        let mut perimeter = 0;

        // check neighbors, add one to perimeter if neighbor is alphabetic and not the same char val as current
        // location
        for dir in self.directions{
            match self.step(coord, dir){
                Some(neighbor) => {
                    let nval = self.get(neighbor);
                    if nval != original && nval != '.' {
                        perimeter += 1;
                    } else if nval == original {
                        // step to next neighbor if neighbor is same char as current
                        let (p, a) = self.analyze(neighbor);
                        perimeter += p;
                        area += a;
                    }
                }
                None => {
                    perimeter += 1;
                }
            }
        }
        // mark with # to indicate region's perimeter and area has already been calculated
        (perimeter, area)
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
    for lat in 0..plot.plot.len(){
        for lon in 0..plot.plot[0].len(){
            let coord = [lat,lon];
            if plot.get(coord).is_alphabetic(){
                let (perim, area) = plot.analyze(coord);
                let region_cost = perim * area;
                sum += region_cost;
                println!("{}Region Cost: {}*{}={}\n", plot, perim, area, region_cost);
                plot.fill(coord,'#');
            }
        }
    }
    println!("Fence cost: {}", sum);
}
