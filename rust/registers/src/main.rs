use std::fs;
use std::env;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Clone)]
struct Instruction {
    index: usize,
    register: String,
    incrementor: i64,
    predicate_actor: String,
    predicate_operator: String,
    predicate_argument: i64,
}

impl Instruction {

    fn parse_register(raw_inst: &str) -> String {
        String::from(
            raw_inst.split(" if ").nth(0).unwrap().trim().split_whitespace()
                .nth(0).unwrap()
        )
    }

    fn parse_incrementor(raw_inst: &str) -> i64 {

        fn match_incrementor(x: &str) -> i64 {
            match x {
                "dec" => -1,
                "inc" => 1,
                _ => i64::from_str(x).expect("Error parsing incrementor"),
            }
        }

        raw_inst.split(" if ").nth(0).unwrap().trim().split_whitespace()
            .skip(1).fold(1, |acc, x| {acc * match_incrementor(x)})
    }

    fn parse_predicate_actor(raw_inst: &str) -> String {
        String::from(
            raw_inst.split(" if ").nth(1).unwrap().trim().split_whitespace()
                .nth(0).unwrap()
        )
    }

    fn parse_predicate_operator(raw_inst: &str) -> String {
        String::from(
            raw_inst.split(" if ").nth(1).unwrap().trim().split_whitespace()
                .nth(1).unwrap()
        )
    }

    fn prase_predicate_argument(raw_inst: &str) -> i64 {
        i64::from_str(
            raw_inst.split(" if ").nth(1).unwrap().trim().split_whitespace()
            .last().unwrap()
        ).expect("Cannot cast predicate argument to int")
    }

    fn parse_instructions() -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        for (i, raw_instruction) in open_file_read_contents().lines().enumerate() {
            instructions.push(
                Instruction {
                    index: i,
                    register: Instruction::parse_register(raw_instruction),
                    incrementor: Instruction::parse_incrementor(raw_instruction),
                    predicate_actor: Instruction::parse_predicate_actor(raw_instruction),
                    predicate_operator: Instruction::parse_predicate_operator(raw_instruction),
                    predicate_argument: Instruction::prase_predicate_argument(raw_instruction),
                }
            );
        };
        instructions
    }

}

fn main() {
    let instructions: Vec<Instruction> = Instruction::parse_instructions();
    let mut registers: HashMap<String, i64> = initialise_registers(&instructions);
    let mut max_process_register: i64 = 0;

    for i in 0..instructions.len() {
        let mut borrow_register = registers.to_owned();
        let instruction: Instruction = get_instruction(&instructions, &i);
        let register_value = registers.entry(instruction.register.to_owned()).or_insert(0);

        if evaluate_predicate(&mut borrow_register, &instruction) {
            let new_value = *register_value + instruction.incrementor;
            max_process_register = if new_value > max_process_register {new_value} else {max_process_register};
            *register_value = new_value;
        }
    };

    // Part A
    let max_end_register = match registers.values().max() {
        Some(x) => x,
        None => panic!("Empty register: If you are seeing this, something has gone terribly wrong."),
    };
    println!("Largest end of instruction register value: {:?}", max_end_register);

    //Part B
    println!("Largest instruction register value: {:?}", max_process_register);
}

fn get_instruction(instructions: &[Instruction], i: &usize) -> Instruction {
    instructions.iter().cloned().find(|instruction| instruction.index == *i).unwrap()
}

fn evaluate_predicate(registers: &mut HashMap<String, i64>, instruction: &Instruction) -> bool {
    let operator = instruction.predicate_operator.as_str();
    let predicate_argument = instruction.predicate_argument;
    let predicate_actor = registers.entry(instruction.predicate_actor.to_owned()).or_insert(0).to_owned();
    let predicate: bool = match operator {
        "==" => predicate_actor == predicate_argument,
        "!=" => predicate_actor != predicate_argument,
        ">=" => predicate_actor >= predicate_argument,
        "<=" => predicate_actor <= predicate_argument,
        ">" => predicate_actor > predicate_argument,
        "<" => predicate_actor < predicate_argument,
        _ => panic!("Unknown operator"),
    };
    predicate
}

fn initialise_registers(instructions: &[Instruction]) -> HashMap<String, i64> {
    let mut registers: HashMap<String, i64> = HashMap::new();
    for instruction in instructions {
        registers.insert(instruction.register.to_owned(), 0);
    };
    registers
}

fn open_file_read_contents() -> String {
    let mut file = match fs::File::open(env::args().nth(1).expect("FAILURE : FILE PATH")) {
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
