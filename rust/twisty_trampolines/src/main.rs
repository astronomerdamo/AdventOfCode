use std::fs;
use std::env;
use std::io::Read;

fn main() {

    let instructions = parse_instructions();
    let instructions_min_length: i32 = 0;
    let instructions_max_length: i32 = safely_parse_usize_to_i32(
        instructions.len()
    );

    // Part A
    find_steps_to_exit_normal(&instructions, &instructions_min_length, &instructions_max_length);

    // Part B
    find_steps_to_exit_abnormal(&instructions, &instructions_min_length, &instructions_max_length);
}

fn find_steps_to_exit_normal(instructions: &[i32], i_min: &i32, i_max: &i32) -> () {
    let mut fn_instructions: Vec<i32> = instructions.to_owned();
    let mut n: i32 = 0;
    let mut s: u32 = 0;

    while n >= *i_min && n < *i_max {

        let instruction = match fn_instructions.get_mut(n as usize) {
            Some(element) => element,
            None => panic!("FAILURE : INSTRUCTION INDEX OUT OF RANGE"),
        };

        n += *instruction;
        *instruction += 1;

        s += 1;
    }

    println!("Normal Steps to Exit: {:?}", s);
}

fn find_steps_to_exit_abnormal(instructions: &[i32], i_min: &i32, i_max: &i32) -> () {
    let mut fn_instructions: Vec<i32> = instructions.to_owned();
    let mut n: i32 = 0;
    let mut s: u32 = 0;

    while n >= *i_min && n < *i_max {

        let instruction = match fn_instructions.get_mut(n as usize) {
            Some(element) => element,
            None => panic!("FAILURE : INSTRUCTION INDEX OUT OF RANGE"),
        };

        n += *instruction;
        *instruction += if *instruction >= 3 {
            -1
        } else {
            1
        };

        s += 1;
    }

    println!("Abnormal Steps to Exit: {:?}", s);
}

fn safely_parse_usize_to_i32(n :usize) -> i32 {
    // I should be able to do this with generics
    if n < std::i32::MAX as usize {
        n as i32
    } else {
        panic!("FAILURE : CANNOT SAFELY CONVERT TYPE")
    }
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
        Err(e) => panic!("FAILURE : OPEN FILE {}", e),
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(s) => s,
        Err(e) => panic!("FAILURE : READ FILE {}", e),
    };
    buffer
}
