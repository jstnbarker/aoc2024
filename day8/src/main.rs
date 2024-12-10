use std::env;
use std::fs;
use std::collections::HashMap;
// must know limits of map
// must know position and frequency of each tower


/* 
 * Return a tuple containing the map limits and a HashMap of tower locations keyed by the
 * tower's frequency
 */
fn load(path: String) -> ([i32;2], HashMap<char, Vec<[i32;2]>>){
    let mut out: HashMap<char, Vec<[i32;2]>> = HashMap::new();
    let content = fs::read_to_string(path).expect("There is no file");
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut j_max = j;
    for char in content.chars() {
        if char == '\n'{
                if j > j_max{
                    j_max = j;
                }
                j = 0;
                i += 1;
        } else if char != '.'{
            let vec = out.get_mut(&char);
            if vec.is_none() {
                out.insert(char, vec![[i,j]]);
            } else{
                vec.unwrap().push([i,j]);
            }
            j+=1;
            
        } else {
            j+=1;
        }

    }
    return ([i-1,j_max-1], out)
}

fn calculate_antinode(a: [i32;2], b: [i32;2], lim:[i32;2]) -> Option<[i32;2]> {
    let i = a[0] + (a[0] - b[0]);
    let j = a[1] + (a[1] - b[1]);
    if j < 0 || i < 0 || i > lim[0] || j > lim[1] {
            return None
    }
    return Some([i,j]);
}

struct Map{
    occupied: Vec<Vec<bool>>
}
impl Map{
    fn new(lim:[i32;2]) -> Self {
        let mut occupied: Vec<Vec<bool>> = Vec::new();
        for _ in 0..(lim[0]+3){
            let mut temp: Vec<bool> = Vec::new();
            for _ in 0..(lim[1]+3){
                temp.push(false);
            }
            occupied.push(temp);
        }
        return Map {
            occupied
        }
    }

    pub fn get(&self, coord: [i32;2]) -> bool {
        return self.occupied[coord[0] as usize][coord[1] as usize];
    }

    pub fn toggle(&mut self, coord: [i32;2]) {
        let state = &mut self.occupied[coord[0] as usize][coord[1] as usize];
        *state = !*state;

    }
    pub fn occupy(&mut self, coord: [i32;2]) {
        self.occupied[coord[0] as usize][coord[1] as usize] = true;

    }
}

// 269 too low
// 278 too high

fn main() {
    let args: Vec<String> = env::args().collect();
    let (limits, towers) = load(args[1].clone());
    let mut m: Map = Map::new(limits);
    let mut total = 0;
    for freq in towers.keys(){
        let tower_list = towers.get(freq).unwrap();
        // do math
        for i in 0..tower_list.len()-1{
            for j in i+1..tower_list.len(){
                let a = tower_list[i];
                let b = tower_list[j];
                let antinode = calculate_antinode(a, b, limits);
                if antinode.is_some(){
                    let antinode = antinode.unwrap();
                    if !m.get(antinode){
                        m.occupy(antinode);
                    }
                }
                let antinode = calculate_antinode(b, a, limits);
                if antinode.is_some(){
                    let antinode = antinode.unwrap();
                    if !m.get(antinode){
                        m.occupy(antinode);
                    }
                }
            }
        }
    }

    for vec in m.occupied{
        for boolean in vec{
            if boolean {
                total+=1;
            }
        }
    }
    println!("{}", total);
}
