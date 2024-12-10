use std::env;
use std::fs;
use std::collections::HashMap;
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
    return ([i-1,j_max], out)
}

fn calculate_antinode(a: [i32;2], b: [i32;2], lim:[i32;2]) -> Option<[i32;2]> {
    let i = a[0] + (a[0] - b[0]);
    let j = a[1] + (a[1] - b[1]);
    if within_limit([i,j],lim){
        return Some([i,j]);
    }
    return None
}

fn within_limit(a:[i32;2],lim:[i32;2]) -> bool {
    if a[0] < 0 || a[1] < 0 || a[0] >= lim[0] || a[1] >= lim[1] {
        return false;
    }
    return true;
}

fn get_antinodes(a:[i32;2],b: [i32;2],lim:[i32;2]) -> Vec<[i32;2]> {
    let mut out: Vec<[i32;2]> = Vec::new();

    let i_offset = a[0] - b[0];
    let j_offset = a[1] - b[1];
    if i_offset == 0 && j_offset == 0{
        return out
    }

    let mut temp = a.clone();
    loop{
        temp = [temp[0]+i_offset, temp[1]+j_offset];
        if within_limit(temp,lim){
            out.push(temp)
        } else {
            break;
        }
    }
    loop{
        temp = [temp[0]-i_offset, temp[1]-j_offset];
        if within_limit(temp,lim){
            out.push(temp);
        } else {
            break;
        }
    }
    return out;
}

struct Map{
    occupied: Vec<Vec<bool>>
}
impl Map{
    fn new(lim:[i32;2]) -> Self {
        let mut occupied: Vec<Vec<bool>> = Vec::new();
        for _ in 0..lim[0]{
            let mut temp: Vec<bool> = Vec::new();
            for _ in 0..lim[1]{
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

    pub fn occupy(&mut self, coord: [i32;2]) {
        self.occupied[coord[0] as usize][coord[1] as usize] = true;
    }

    fn get_occupied(&self) -> i32 {
        let mut total = 0;
        for i in 0..self.occupied.len(){
            for j in 0..self.occupied[i].len(){
                if self.get([i as i32,j as i32]){
                    total+=1;
                }
            }
        }
        return total;
    }
}
//part 2 971 too low
//part 2 972 too low

fn main() {
    let args: Vec<String> = env::args().collect();
    let (limits, towers) = load(args[1].clone());
    let mut p1: Map = Map::new(limits);
    let mut p2: Map = Map::new(limits);
    for freq in towers.keys(){
        let tower_list = towers.get(freq).unwrap();

        /* PART 1 */
        for i in 0..tower_list.len()-1{
            for j in i+1..tower_list.len(){
                let a = tower_list[i];
                let b = tower_list[j];
                let antinode = calculate_antinode(a, b, limits);
                if antinode.is_some(){
                    p1.occupy(antinode.unwrap());
                }
                let antinode = calculate_antinode(b, a, limits);
                if antinode.is_some(){
                    p1.occupy(antinode.unwrap());
                }
            }
        }

        /* PART 2 */
        for i in 0..tower_list.len()-1{
            for j in i..tower_list.len(){
                for coord in get_antinodes(tower_list[i],tower_list[j],limits){
                    p2.occupy(coord)
                }
            }
        }
    }
    println!("{}", p1.get_occupied());
    println!("{}", p2.get_occupied());
}
