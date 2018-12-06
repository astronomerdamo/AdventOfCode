use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use utils;

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
            raw_inst
                .split(" if ")
                .nth(0)
                .unwrap()
                .trim()
                .split_whitespace()
                .nth(0)
                .unwrap(),
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

        raw_inst
            .split(" if ")
            .nth(0)
            .unwrap()
            .trim()
            .split_whitespace()
            .skip(1)
            .fold(1, |acc, x| acc * match_incrementor(x))
    }

    fn parse_predicate_actor(raw_inst: &str) -> String {
        String::from(
            raw_inst
                .split(" if ")
                .nth(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .nth(0)
                .unwrap(),
        )
    }

    fn parse_predicate_operator(raw_inst: &str) -> String {
        String::from(
            raw_inst
                .split(" if ")
                .nth(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .nth(1)
                .unwrap(),
        )
    }

    fn prase_predicate_argument(raw_inst: &str) -> i64 {
        i64::from_str(
            raw_inst
                .split(" if ")
                .nth(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .last()
                .unwrap(),
        ).expect("Cannot cast predicate argument to int")
    }
}

fn load_raw_input() -> String {
    let path = Path::new("./inputs/year2017day8.txt");
    println!("Loading inputs file: {:?}", &path);
    utils::file_utils::read_inputs(path)
}

fn parse_input() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for (i, raw_instruction) in load_raw_input().lines().enumerate() {
        instructions.push(Instruction {
            index: i,
            register: Instruction::parse_register(raw_instruction),
            incrementor: Instruction::parse_incrementor(raw_instruction),
            predicate_actor: Instruction::parse_predicate_actor(raw_instruction),
            predicate_operator: Instruction::parse_predicate_operator(raw_instruction),
            predicate_argument: Instruction::prase_predicate_argument(raw_instruction),
        });
    }
    instructions.sort_by(|a, b| a.index.cmp(&b.index));
    instructions
}

fn evaluate_predicate(predicate_actor: i64, instruction: &Instruction) -> bool {
    let operator = instruction.predicate_operator.as_str();
    let predicate_argument = instruction.predicate_argument;
    match operator {
        "==" => predicate_actor == predicate_argument,
        "!=" => predicate_actor != predicate_argument,
        ">=" => predicate_actor >= predicate_argument,
        "<=" => predicate_actor <= predicate_argument,
        ">" => predicate_actor > predicate_argument,
        "<" => predicate_actor < predicate_argument,
        _ => panic!("Unknown operator"),
    }
}

fn run_instructions(instructions: &[Instruction]) -> (HashMap<String, i64>, i64) {
    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut max_register_value: i64 = 0;
    for instruction in instructions {
        if evaluate_predicate(
            *registers
                .entry(instruction.predicate_actor.to_owned())
                .or_insert(0),
            instruction,
        ) {
            let value = registers
                .entry(instruction.register.to_owned())
                .or_insert(0);
            let new_value = *value + instruction.incrementor;
            *value = new_value;
            max_register_value = if new_value > max_register_value {
                new_value
            } else {
                max_register_value
            };
        };
    }
    (registers, max_register_value)
}

fn run_a(res: &(HashMap<String, i64>, i64)) -> i64 {
    res.0.values().max().unwrap().to_owned()
}

fn run_b(res: &(HashMap<String, i64>, i64)) -> i64 {
    res.1
}

pub fn main() {
    let input: Vec<Instruction> = parse_input();
    let res = run_instructions(&input);
    println!("Day 8a: {:?}", &run_a(&res));
    println!("Day 8a: {:?}", &run_b(&res));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_instructions() -> Vec<Instruction> {
        vec![
            Instruction {
                index: 1,
                register: "b".to_string(),
                incrementor: 5,
                predicate_actor: "a".to_string(),
                predicate_operator: ">".to_string(),
                predicate_argument: 1,
            },
            Instruction {
                index: 2,
                register: "a".to_string(),
                incrementor: 1,
                predicate_actor: "b".to_string(),
                predicate_operator: "<".to_string(),
                predicate_argument: 5,
            },
            Instruction {
                index: 3,
                register: "c".to_string(),
                incrementor: 10,
                predicate_actor: "a".to_string(),
                predicate_operator: ">=".to_string(),
                predicate_argument: 1,
            },
            Instruction {
                index: 4,
                register: "c".to_string(),
                incrementor: -20,
                predicate_actor: "c".to_string(),
                predicate_operator: "==".to_string(),
                predicate_argument: 10,
            },
        ]
    }

    #[test]
    fn test_run_a() {
        let test_res = run_instructions(&test_instructions());
        assert_eq!(run_a(&test_res), 1);
    }

    #[test]
    fn test_run_b() {
        let test_res = run_instructions(&test_instructions());
        assert_eq!(run_b(&test_res), 10);
    }

}
