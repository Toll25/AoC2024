use std::{collections::HashMap, fs::read_to_string};

use itertools::{repeat_n, Itertools};

#[derive(Debug)]
enum Op {
    Mult,
    Add,
    Concat,
}

fn main() {
    let calcs = get_calcs();

    let mut counter = 0;
    'a: for (result, nums) in calcs.clone() {
        let options: Vec<Vec<&Op>> =
            repeat_n([Op::Mult, Op::Add, Op::Concat].iter(), nums.len() - 1)
                .multi_cartesian_product()
                .collect();
        //dbg!(nums.len() - 1);
        //dbg!(&nums);
        //dbg!(&options);

        for option in options {
            let mut test_result = 0;
            for (index, num) in nums.iter().enumerate() {
                if index == 0 {
                    test_result = *num;
                    continue;
                }
                match option.get(index - 1).unwrap() {
                    Op::Mult => {
                        test_result *= num;
                    }
                    Op::Add => {
                        test_result += num;
                    }
                    Op::Concat => {
                        test_result = concat(test_result as u64, *num as u64) as i64;
                    }
                }
            }
            if test_result == result {
                //solved
                counter += result;
                continue 'a;
            }
        }
        // not solved
    }
    println!("Num of calculations: {}", calcs.len());
    println!("Num of possble solves: {counter}");

    // (+,*).premuations(num of calcs)
}
fn concat(a: u64, b: u64) -> u64 {
    a as u64 * 10u64.pow(b.ilog10() + 1) + b as u64
}

fn get_calcs() -> HashMap<i64, Vec<i64>> {
    let content = read_to_string("data.txt").unwrap();

    let mut calcs: HashMap<i64, Vec<i64>> = HashMap::new();
    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }
        let split: Vec<String> = line
            .split(": ")
            .map(std::string::ToString::to_string)
            .collect();
        //dbg!(split.first().unwrap());
        let result = split.first().unwrap().parse::<i64>().unwrap();
        let nums_el = split.last().unwrap();
        let nums: Vec<i64> = nums_el
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        calcs.insert(result, nums);
    }
    calcs
}
