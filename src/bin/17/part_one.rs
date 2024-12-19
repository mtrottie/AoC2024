use core::panic;
use std::fs::read_to_string;

const NUM: &str = "17";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    fn parse(c: i32) -> OpCode {
        match c {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => panic!("Invalid operand"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Operand;

impl Operand {
    fn combo(operand: i32, register_a: i32, register_b: i32, register_c: i32) -> i32 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            7 | _ => panic!("Invalid operand"),
        }
    }
}

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> String {
    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut instructions = vec![];

    for line in read_to_string(file_path).unwrap().lines() {
        if line.contains("Register A") {
            register_a = line.split_once("A: ").unwrap().1.parse::<i32>().unwrap();
        }
        if line.contains("Register B") {
            register_b = line.split_once("B: ").unwrap().1.parse::<i32>().unwrap();
        }
        if line.contains("Register C") {
            register_c = line.split_once("C: ").unwrap().1.parse::<i32>().unwrap();
        }
        if line.contains("Program") {
            instructions = line
                .split_once(": ")
                .unwrap()
                .1
                .split(',')
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
        }
    }

    let mut output = vec![];
    let mut instruction = 0;
    while instruction < instructions.len() {
        let op_code_number = instructions[instruction];
        let op_code = OpCode::parse(op_code_number);
        let operand = instructions[instruction + 1];
        let combo = Operand::combo(operand, register_a, register_b, register_c);

        println!("OpCode Number: {op_code_number}, Opcode: {:?}, Operand: {:?}, Combo: {:?}, registerA: {register_a}, registerB: {register_b}, registerC: {register_c}", op_code, operand, combo);
        println!("Output: {:?}", output);

        match op_code {
            OpCode::Adv => register_a = register_a.clone() / (2 as i32).pow(combo as u32),
            OpCode::Bxl => register_b = register_b.clone() ^ operand,
            OpCode::Bst => register_b = combo % 8,
            OpCode::Jnz => {
                if register_a.clone() != 0 {
                    instruction = operand as usize;
                    continue;
                }
            }
            OpCode::Bxc => register_b = register_b.clone() ^ register_c.clone(),
            OpCode::Out => {
                output.push(combo % 8);
            }
            OpCode::Bdv => register_b = register_a.clone() / (2 as i32).pow(combo as u32),
            OpCode::Cdv => register_c = register_a.clone() / (2 as i32).pow(combo as u32),
        }

        instruction += 2;
    }

    println!("Output: {:?}", output);
    output
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(
            super::part(
                format!(
                    "/home/phobos/code/advent_2024/src/bin/{}/data_test.txt",
                    super::NUM
                )
                .as_str()
            ),
            "4,6,3,5,6,3,5,2,1,0".to_string()
        );
    }

    #[test]
    fn baby_tests() {
        assert_eq!(
            super::part(
                format!(
                    "/home/phobos/code/advent_2024/src/bin/{}/data_test_2.txt",
                    super::NUM
                )
                .as_str()
            ),
            "0,1,2".to_string()
        )
    }

    #[test]
    fn baby_test_two() {
        assert_eq!(
            super::part(
                format!(
                    "/home/phobos/code/advent_2024/src/bin/{}/data_test_3.txt",
                    super::NUM
                )
                .as_str()
            ),
            "4,2,5,6,7,7,7,7,3,1,0".to_string()
        )
    }
}
