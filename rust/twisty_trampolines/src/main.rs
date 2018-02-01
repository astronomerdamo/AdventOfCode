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
    let snorm: u32 = find_steps_to_exit_normal(&instructions, &instructions_min_length, &instructions_max_length);
    println!("Normal Steps to Exit: {:?}", snorm);

    // Part B
    let sabn: u32 = find_steps_to_exit_abnormal(&instructions, &instructions_min_length, &instructions_max_length);
    println!("Abnormal Steps to Exit: {:?}", sabn);
}

fn find_steps_to_exit_normal(instructions: &[i32], i_min: &i32, i_max: &i32) -> (u32) {
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

    s
}

fn find_steps_to_exit_abnormal(instructions: &[i32], i_min: &i32, i_max: &i32) -> (u32) {
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

    s
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safely_convert_usize() {
        let test_n: usize = 42;
        let expected_n: i32 = 42;
        assert_eq!(safely_parse_usize_to_i32(test_n), expected_n);
    }

    #[test]
    #[should_panic]
    fn panic_convert_usize() {
        let test_n: usize = std::i32::MAX as usize + 1;
        safely_parse_usize_to_i32(test_n);
    }

    #[test]
    fn test_find_steps_to_exit_normal() {
        let test_instr: [i32; 5] = [0, 3, 0, 1, -3];
        let test_instr_min_length: i32 = 0;
        let test_instr_max_length: i32 = 5;
        let expected = 5;
        assert_eq!(
            find_steps_to_exit_normal(&test_instr, &test_instr_min_length, &test_instr_max_length),
            expected
        )
    }

    #[test]
    fn test_find_steps_to_exit_abnormal() {
        let test_instr: [i32; 5] = [0, 3, 0, 1, -3];
        let test_instr_min_length: i32 = 0;
        let test_instr_max_length: i32 = 5;
        let expected = 10;
        assert_eq!(
            find_steps_to_exit_abnormal(&test_instr, &test_instr_min_length, &test_instr_max_length),
            expected
        )
    }
}
