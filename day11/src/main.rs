#[derive(Eq, PartialEq, Hash)]
struct Stone{
    val: u64,
    quantity: u64,
}

impl Stone{
    fn new(val: u64) -> Self {
        Stone {
            val,
            quantity: 1,
        }
    }

    fn split(&mut self) -> Option<Self> {
        let stringval = self.val.to_string();
        if self.val == 0 {
            self.val = 1;
        } else if stringval.len()%2 == 0 {
            let (l, r) = stringval.split_at(stringval.len()/2);
            self.val = l.parse().expect("Failed to parse");
            return Some(Stone{val: r.parse().expect("Failed to parse"), quantity: self.quantity})
        } else {
            self.val = self.val * 2024;
        }
        return None;
    }
}
impl Clone for Stone{
    fn clone(&self) -> Self {
        Stone{
            val: self.val,
            quantity: self.quantity
        }
    }
}

use std::collections::HashMap;
struct Stonehenge{
    stones: HashMap<u64, Stone>,
}

impl Stonehenge{
    fn new(stone_string: String) -> Self {
        // use stone's val as hash to hashmap, and store whole stone at hashed val
        let mut stones: HashMap<u64, Stone> = HashMap::new();
        let mut iter = stone_string.chars();
        let mut _builder = "".to_string();
        loop {
            match iter.next(){
                Some(c) => {
                    if c == ' ' || c == '\n' {
                        match _builder.parse::<u64>(){
                            Ok(val) => {
                                stones.insert(val, Stone::new(val));
                                _builder = "".to_string();
                            }
                            Err(_) => {}
                        }
                    } else {
                        _builder.push(c);
                    }
                }
                None => {
                    return Stonehenge{stones};
                }
            }
        }
    }

    fn blink(&mut self){
        let mut post_blink: HashMap<u64,Stone> = HashMap::new();

        let mut stone_iter = self.stones.values_mut();
        loop{
            match stone_iter.next() {
                Some(stone) => {
                    // apply rules to current stone
                    match stone.split(){ 
                        Some(split) => {
                            // if a sister stone is returned from stone:
                            match post_blink.get_mut(&split.val){
                                // increment the preexisting stone's quantity value
                                Some(stone) => {
                                    stone.quantity+=split.quantity;
                                }
                                // add a new key value pair for the sister to the map
                                None => {
                                    post_blink.insert(split.val, split);
                                }
                            }
                        }
                        // do nothing if the call to split doesn't return a sister stone 
                        // as the stone that called to be split has modified itself according to
                        // the rules
                        None => {}
                    }
                    // because stone.split modifies the calling stone directly, optionally
                    // returning a new stone (see above), we gotta add the (now modified) 
                    // stone's values to the new map
                    //
                    // i should probably make this a function rather than repeating myself 
                    // but the post_blink map would be out of that function's scope unless 
                    // i add a new value to the struct 
                    match post_blink.get_mut(&stone.val){
                        Some(value) => {
                            value.quantity+=stone.quantity;
                        }
                        None => {
                            // cloning as a workaround to the lsp and compiler complaints
                            let copy = stone.clone();
                            post_blink.insert(stone.val, copy);
                        }
                    }
                }
                // break out of loop when the iterator runs out of stones to split
                None => {
                    break;
                }
            }
        }
        self.stones = post_blink;
    }

    fn count_stones(&self) -> u64 {
        let mut stone_iter = self.stones.values();
        let mut sum = 0;
        loop{
            match stone_iter.next(){
                Some(stone) => {
                    sum += stone.quantity;
                }
                None => {
                    break;
                }
            }
        }
        sum
    }
}

use std::fmt;
impl fmt::Display for Stonehenge{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = "".to_string();
        let mut stone_iter = self.stones.values();
        loop{
            match stone_iter.next(){
                Some(stone) => {
                    display += &stone.val.to_string();
                    display.push(' ')
                }
                None => {
                    break;
                }
            }
        }
        write!(f, "{}", display)
    }
}
use std::fs;
fn load(path: String) -> String {
    fs::read_to_string(path).expect("File not found")
}

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_content = load(args[1].clone());
    let mut row = Stonehenge::new(file_content);
    for i in 0..args[2].clone().parse().unwrap(){
        //println!("Blink #{}", i);
        //println!("{}", row);
        row.blink()
    }
    println!("# of stones: {}", row.count_stones());
}

