use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("data.txt").expect("Should have been able to read the file");
    let outer_regex = Regex::new(r"((\A)|(do\(\)))(.|\n)*?((\z)|(don't\(\)))").unwrap();
    let inner_regex = Regex::new(r"mul\((?<num1>[0-9]{1,3}),(?<num2>[0-9]{1,3})\)").unwrap();
    let dos = outer_regex.find_iter(&contents);
    let mut numbers: Vec<(i32, i32)> = Vec::new();
    for segment in dos {
        println!("{segment:#?}");
        let mut matches: Vec<(i32, i32)> = inner_regex
            .captures_iter(segment.as_str())
            .map(|caps| {
                let num1 = caps.name("num1").unwrap().as_str().parse::<i32>().unwrap();
                let num2 = caps.name("num2").unwrap().as_str().parse::<i32>().unwrap();
                (num1, num2)
            })
            .collect();
        numbers.append(&mut matches);
    }
    let mut result = 0;
    for (num1, num2) in numbers {
        result += num1 * num2;
    }
    println!("{result:#?}");
}
