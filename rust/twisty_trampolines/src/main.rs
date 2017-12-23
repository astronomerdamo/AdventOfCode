use std::fs;
use std::env;
use std::io::Read;

fn main() {

    let mut instructions = parse_instructions();

    let min_length: i32 = 0;
    let max_length: i32 = instructions.len() as i32;
    let mut n: i32 = 0;
    let mut s: u32 = 0;

    while n >= min_length && n < max_length {

        let instruction = match instructions.get_mut(n as usize) {
            Some(element) => {
                // left in for debugging
                // println!("{:?}", element);
                element
            },
            None => panic!("OUT OF FUCKING RANGE"),
        };

        n += *instruction;
        *instruction += 1;

        s += 1;
    }

    println!("Steps: {:?}", s);

}

fn parse_instructions() -> Vec<i32> {
    open_file_read_contents()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn open_file_read_contents() -> String {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => panic!("FAILURE : FILE PATH"),
    };

    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => panic!("FAILURE : OPEN FILE"),
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(s) => s,
        Err(_) => panic!("FAILURE : READ FILE"),
    };
    buffer
}

