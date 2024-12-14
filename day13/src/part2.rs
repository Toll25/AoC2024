use core::f64;
use regex::Regex;
use std::fs::read_to_string;

pub fn run() {
    let claw_data = get_claw_data();
    let mut count = 0;
    for claw in claw_data {
        let v1 = claw.a.0 as f64;
        let v2 = claw.b.0 as f64;
        let v3 = claw.result.0 as f64;
        let v4 = claw.a.1 as f64;
        let v5 = claw.b.1 as f64;
        let v6 = claw.result.1 as f64;

        dbg!(v3, v6);

        let y: f64 = ((v3 * v4) - (v1 * v6)) / ((v2 * v4) - (v1 * v5));
        let x: f64 = (v3 - (y * v2)) / v1;
        dbg!(x, y);
        if x.fract() == 0. && y.fract() == 0. {
            println!("passed");
            println!("{} {}", x, y);
            println!("{} {}", x as i64, y as i64);
            count += 3 * x as i64 + y as i64;
        }
    }
    dbg!(count);
}

pub struct ClawData {
    pub a: (i64, i64),
    pub b: (i64, i64),
    pub result: (i64, i64),
}
fn get_claw_data() -> Vec<ClawData> {
    let content = read_to_string("data.txt").unwrap();

    let button_regex = Regex::new(r"X\+(?<cord1>[0-9]{2}).*(?<cord2>[0-9]{2})").unwrap();
    let result_regex = Regex::new(r"X=(?<cord1>[0-9]*).*?Y=(?<cord2>[0-9]*)").unwrap();

    let mut button_values: Vec<(i64, i64)> = button_regex
        .captures_iter(&content)
        .map(|caps| {
            let num1 = caps.name("cord1").unwrap().as_str().parse::<i64>().unwrap();
            let num2 = caps.name("cord2").unwrap().as_str().parse::<i64>().unwrap();
            (num1, num2)
        })
        .collect();
    let result_values: Vec<(i64, i64)> = result_regex
        .captures_iter(&content)
        .map(|caps| {
            let num1 = caps.name("cord1").unwrap().as_str().parse::<i64>().unwrap();
            let num2 = caps.name("cord2").unwrap().as_str().parse::<i64>().unwrap();
            (num1 + 10_000_000_000_000, num2 + 10_000_000_000_000)
            //(num1, num2)
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
