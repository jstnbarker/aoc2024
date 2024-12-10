use std::fs;
use std::env;

fn load(path: String) -> String {
    return fs::read_to_string(path).expect("no file");
}

fn expand(disk_map: String) -> Vec<i32>{
    let mut out: Vec<i32> = Vec::new();
    let mut id = 0;

    let chars: Vec<char> = disk_map.chars().collect();
    for i in 0..chars.len()-1{
        let block_count:i32 = chars[i] as i32 - 0x30;
        let mut push_val = -1;
        if i % 2 == 0{
            push_val = id;
            id+=1;
        }
        for _ in 0..block_count{
            out.push(push_val);
        }
    }

    return out;
}

fn defrag(mut disk: Vec<i32>)->Vec<i32>{
    let mut rptr = disk.len()-1;
    let mut lptr = 0;
    loop{
        if disk[lptr] == -1{
            loop{
                if disk[rptr] != -1{
                    disk.swap(lptr,rptr);
                    break;
                }
                rptr-=1;
                if rptr < lptr {
                    break;
                }
            }
        }
        lptr += 1;
        if lptr > rptr{
            break;
        }
    }
    return disk
}

fn main() {
    let mut args = env::args();
    args.next();
    let disk = load(args.next().unwrap());
    let result = defrag(expand(disk));
    let mut sum = 0;
    for i in 0..result.len(){
        if result[i] < 0{
            break;
        }
        sum += i * result[i] as usize 
    }
    println!("{}", sum);
}
