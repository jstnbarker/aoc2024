use std::fs;
use std::env;
use std::collections::LinkedList;

// -1 indicates free space
// anything else indicates a file
struct File {
    id: i32,
    len: usize,
}

fn load(path: String) -> String {
    return fs::read_to_string(path).expect("no file");
}

fn expand(disk_map: String) -> LinkedList<File>{
    let mut out: LinkedList<File> = LinkedList::new();
    let mut id = 0;

    let chars: Vec<char> = disk_map.chars().collect();
    for i in 0..chars.len()-2{
        let block_count: usize = (chars[i] as i32 - 0x30) as usize;
        let mut file_id = -1;
        if i % 2 == 0{
            file_id = id;
            id+=1;
        }
        out.push_back(File{id:file_id, len:block_count});
    }
    return out;
}

fn defrag(mut disk: LinkedList<File>)->LinkedList<File>{
    let mut rptr = disk.back().iter().rev();
    let mut lptr = disk.iter();
    loop{
        let lptr_file = lptr.next().unwrap();
        if lptr_file.id == -1 {
        }
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let disk = expand(load(args.next().unwrap()));
    for file in disk {
        println!("id: {:2}, len: {}", file.id, file.len);
    }
    /*
    let result = defrag(disk);
    let mut sum = 0;
    for val in result {
        if val.id >= 0{
            sum += val.id as usize * val.len

        }
    }
    println!("{}", sum);
    */
}
