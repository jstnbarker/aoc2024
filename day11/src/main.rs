struct Stone{
    val: u64,
}

impl Stone{
    fn new(val: u64) -> Self {
        Stone {
            val
        }
    }
    fn split(&mut self) -> Option<Self> {
        let stringval = self.val.to_string();
        if self.val == 0 {
            self.val = 1;
        } else if stringval.len()%2 == 0 {
            let (l, r) = stringval.split_at(stringval.len()/2);
            self.val = l.parse().expect("Failed to parse");
            return Some(Stone{val: r.parse().expect("Failed to parse")})
        } else {
            self.val = self.val * 2024;
        }
        return None;
    }
}
struct Stonehenge{
    stones: Vec<Stone>,
}

impl Stonehenge{
    fn new(stone_string: String) -> Self {
        let mut stones: Vec<Stone> = Vec::new();
        let mut iter = stone_string.chars();
        let mut _builder = "".to_string();
        loop {
            match iter.next(){
                Some(c) => {
                    if c == ' ' || c == '\n' {
                        match _builder.parse::<u64>(){
                            Ok(val) => {
                                stones.push(Stone::new(val));
                                _builder = "".to_string();
                            }
                            Err(_) => {}
                        }
                    } else {
                        _builder.push(c);
                    }
                }
                None => {
                    stones.reverse();
                    return Stonehenge{stones};
                }
            }
        }
    }

    fn blink(&mut self){
        let mut i = 0;
        while i < self.stones.len(){
            match self.stones[i].split(){
                Some(new_stone) => {
                    self.stones.insert(i+1, new_stone);
                    i+=1;
                }
                None => {}
            }
            i+=1;
        }
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
        println!("Blink #{}", i);
        row.blink()
    }
    println!("# of stones: {}", row.stones.len());
}

