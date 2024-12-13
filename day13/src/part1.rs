use regex::Regex;
use std::fs::read_to_string;

pub fn run() {
    let claw_data = get_claw_data();

    let mut value = 0;
    for claw in &claw_data {
        let mut lowest = 1000000000;
        let mut counter = 0;
        for i in 0..=100 {
            for j in 0..=100 {
                let cord1 = claw.a.0 * i + claw.b.0 * j;
                let cord2 = claw.a.1 * i + claw.b.1 * j;
                if cord1 > claw.result.0 || cord2 > claw.result.1 {
                    continue;
                } else if cord1 == claw.result.0 && cord2 == claw.result.1 {
                    let tokens = i * 3 + j;
                    println!("A: {}", i * 3);
                    println!("B: {}", j);
                    if tokens < lowest {
                        lowest = tokens;
                        counter += tokens;
                    }
                }
            }
        }
        value += counter;
    }
    dbg!(value);

    //dbg!(claw_data);
}

#[derive(Debug)]
pub struct ClawData {
    pub a: (i32, i32),
    pub b: (i32, i32),
    pub result: (i32, i32),
}
fn get_claw_data() -> Vec<ClawData> {
    let content = read_to_string("data.txt").unwrap();

    let button_regex = Regex::new(r"X\+(?<cord1>[0-9]{2}).*(?<cord2>[0-9]{2})").unwrap();
    let result_regex = Regex::new(r"X=(?<cord1>[0-9]*).*?Y=(?<cord2>[0-9]*)").unwrap();

    let mut button_values: Vec<(i32, i32)> = button_regex
        .captures_iter(&content)
        .map(|caps| {
            let num1 = caps.name("cord1").unwrap().as_str().parse::<i32>().unwrap();
            let num2 = caps.name("cord2").unwrap().as_str().parse::<i32>().unwrap();
            (num1, num2)
        })
        .collect();
    let result_values: Vec<(i32, i32)> = result_regex
        .captures_iter(&content)
        .map(|caps| {
            let num1 = caps.name("cord1").unwrap().as_str().parse::<i32>().unwrap();
            let num2 = caps.name("cord2").unwrap().as_str().parse::<i32>().unwrap();
            (num1, num2)
        })
        .collect();

    button_values.reverse();
    let mut claw_data = Vec::new();
    for result in result_values {
        claw_data.push(ClawData {
            a: button_values.pop().unwrap(),
            b: button_values.pop().unwrap(),
            result,
        });
    }
    claw_data
}
