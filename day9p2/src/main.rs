use std::fs;
use std::env;

// -1 indicates free space
// anything else indicates a file
#[derive(Clone)]
struct File { 
    id: usize,
    len: usize,
    free: usize,
}

fn load(path: String) -> String {
    fs::read_to_string(path).expect("no file").to_string()
}

fn expand(disk_map: String) -> Vec<File>{
    let mut out: Vec<File> = Vec::new();
    let mut id = 0;
    let mut char_iter = disk_map.chars();
    loop{
        match char_iter.next(){
            Some(file_size) => {
                let file_size: usize = (file_size as i32 - 0x30) as usize;
                let mut free:usize = 0;
                match char_iter.next(){
                    Some(free_length) =>{
                        free = (free_length as i32 - 0x30) as usize;
                    }
                    None => break
                }
                if free > usize::max_value()-2000{
                    free = 0;
                }
                out.push(File{id, len: file_size, free});
                id+=1;
            }
            None => break
        }
    } 
    let last = out.len()-1;
    out[last].free=0;
    out
}

fn defrag(mut disk: Vec<File>)->Vec<File>{
    let mut r = disk.len()-1;
    while r > 0{
        let mut swapped = false;
        let mut l = 0;
        while l < r {
            if disk[l].free >= disk[r].len{
                /* sum right's neighbor's free space and the current file's free space and size */
                disk[r-1].free += disk[r].len+disk[r].free;
                /* set right file's free to left's free minus right's size left file */
                disk[r].free = disk[l].free - disk[r].len;
                /* set left file's free space to zero */
                disk[l].free = 0;

                /* insert right file after left file */
                let temp = disk.remove(r);
                disk.insert(l+1, temp);
                 
                swapped = true;
                break;
            }
            l+=1;
        }
        if !swapped {
            r-=1;
        }
    }
    disk
}

fn main() {
    let max:usize = 10011500126037;
    let mut args = env::args();
    args.next();
    let disk = expand(load(args.next().unwrap()));
    let result = defrag(disk);
    let mut next_file_start:usize = 0;
    let mut sum: usize = 0;
    for file in result {
        //println!("id {:4}, size {:4}, free {:10}", file.id, file.len, file.free);
        for i in next_file_start..next_file_start+file.len {
            sum = sum + (file.id * i);
        }
        next_file_start += file.len+file.free;
    }
    if sum >= max {
        println!("Answer is too large");
    }
    println!("{}", sum);
}
