use std::fs;
use std::env;
use std::io::Read;

struct Instruction {
    register: String,
    incrementor: i64,
    predicate_actor: String,
    predicate_comparator: String,
    predicate_argument: i64,
}

fn main() {
    let registers = open_file_read_contents();
    let predicate: Vec<String> = parse_predicate(&registers);
    println!("{:?}", &predicate);
    println!("{:?}", &registers);
}

fn parse_predicate(registers: &str) -> Vec<String> {
    registers
        .lines()
        .map(|line| String::from(line.split("if").nth(1).unwrap().trim()))
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
