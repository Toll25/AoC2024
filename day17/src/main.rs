use core::panic;
use std::{cmp::min, fs::read_to_string};

use regex::Regex;

#[derive(Debug, Clone)]
struct State {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    program: Vec<u8>,
}

fn simulate(reg_a: i64, program: &Vec<u8>) -> Vec<u8> {
    let program = program.clone();
    let mut state = State {
        reg_a,
        reg_b: 0,
        reg_c: 0,
        program,
    };
    let mut index = 0;
    let mut output = Vec::new();
    loop {
        if let Some(opcode) = state.program.get(index) {
            if let Some(operand) = state.program.get(index + 1) {
                //println!("Opcode and Operand: {} {}", opcode, operand);
                match opcode {
                    0 => {
                        let operand = get_combo(&state, *operand);
                        state.reg_a = (state.reg_a as f64
                            / 2_i64.pow(operand.try_into().unwrap()) as f64)
                            .trunc() as i64;
                    }
                    1 => {
                        state.reg_b ^= i64::from(*operand);
                    }
                    2 => {
                        state.reg_b = get_combo(&state, *operand) % 8;
                    }
                    3 => {
                        if state.reg_a != 0 {
                            index = *operand as usize;
                            continue;
                        }
                    }
                    4 => {
                        state.reg_b ^= state.reg_c;
                    }
                    5 => {
                        output.push((get_combo(&state, *operand) % 8).try_into().unwrap());
                    }
                    6 => {
                        let operand = get_combo(&state, *operand);
                        state.reg_b = (state.reg_a as f64
                            / 2_i64.pow(operand.try_into().unwrap()) as f64)
                            .trunc() as i64;
                    }
                    7 => {
                        let operand = get_combo(&state, *operand);
                        state.reg_c = (state.reg_a as f64
                            / 2_i64.pow(operand.try_into().unwrap()) as f64)
                            .trunc() as i64;
                    }
                    _ => {
                        panic!("Invalid operation");
                    }
                }
            } else {
                break;
            }
        } else {
            break;
        }
        index += 2;
    }
    output
}

//private fun findPossibleStarting(registerA: Long, expectedOutput: Long, state: State): List<Long> {
//    val possibleRegisterA = mutableListOf<Long>()
//    for (contender in 0L..7L) {
//        val contenderRegisterA = (registerA shl 3) or contender
//        if (state.getFirstOutputFor(contenderRegisterA) == expectedOutput) {
//            possibleRegisterA.add(contenderRegisterA)
//        }
//    }
//    return possibleRegisterA
//}

fn find_possible_starting(reg_a: i64, expected: u8, state: &State) -> Vec<i64> {
    let mut possi = Vec::new();
    for maybe in 0..8 {
        let maybe_calc = (reg_a << 3) | maybe;
        if (simulate(maybe_calc, &state.program)).first().unwrap() == &expected {
            println!("pushing");
            possi.push(maybe_calc);
        }
    }
    possi
}

fn main() {
    let state = get_program();
    dbg!(&state);

    let mut possible_starting_values = vec![0];
    let mut reversed_program = state.program.clone();
    reversed_program.reverse();
    for instruction in &reversed_program {
        dbg!(instruction);
        let temp = possible_starting_values.clone();
        for value in temp {
            possible_starting_values.extend(find_possible_starting(value, *instruction, &state));
        }
    }
    dbg!(possible_starting_values);
    dbg!(simulate(236581108670061, &state.program));
}

fn get_combo(state: &State, operand: u8) -> i64 {
    match operand {
        0..=3 => i64::from(operand),
        4 => state.reg_a,
        5 => state.reg_b,
        6 => state.reg_c,
        _ => {
            panic!("Invalid operand");
        }
    }
}

fn get_program() -> State {
    let content = read_to_string("data.txt").unwrap();
    let register_regex = Regex::new(r"Register .: (?<num>\d*)").unwrap();
    let program_line_regex = Regex::new(r"Program: (.*)").unwrap();
    let digit_regex = Regex::new(r"\d").unwrap();

    let register_values: Vec<i64> = register_regex
        .captures_iter(&content)
        .map(|caps| {
            let num = caps.name("num").unwrap().as_str().parse::<i64>().unwrap();
            num
        })
        .collect();

    let program_line = program_line_regex
        .captures(&content)
        .unwrap()
        .get(1) // Access the first capture group (.*)
        .unwrap()
        .as_str();

    let digits: Vec<u8> = digit_regex
        .find_iter(program_line) // Use `find_iter` to iterate over matches
        .map(|digit| digit.as_str().parse::<u8>().unwrap()) // Convert matched string to u8
        .collect();

    State {
        reg_a: register_values[0],
        reg_b: register_values[1],
        reg_c: register_values[2],
        program: digits,
    }
}
