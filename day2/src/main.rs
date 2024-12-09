use std::{
    cmp::{max, min},
    fs::read_to_string,
};

fn main() {
    let lines: Vec<String> = read_to_string("data.txt")
        .unwrap()
        .lines()
        .map(String::from) // make each slice into a string
        .collect();

    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let split = line.split_whitespace();
        let mut report: Vec<i32> = Vec::new();
        for number in split {
            report.push(number.parse::<i32>().unwrap());
        }
        reports.push(report);
    }

    let mut num_of_correct = 0;
    let mut check_again: Vec<Vec<i32>> = Vec::new();
    for report in reports {
        if check_check(report.clone()) {
            num_of_correct += 1;
        } else {
            check_again.push(report);
        }
    }

    'check: for report in check_again {
        for (skip_num, _) in report.iter().enumerate() {
            let mut report_clone = report.clone();
            report_clone.remove(skip_num);
            if check_check(report_clone) {
                num_of_correct += 1;
                continue 'check;
            }
        }
    }

    println!("Number of safe: {num_of_correct}");
}

fn check_check(report: Vec<i32>) -> bool {
    let mut last_number: i32 = *report.first().unwrap();
    let mut rising_or_falling = 0;
    for num in &report[1..] {
        if num < &last_number {
            if rising_or_falling == 1 {
                return false;
            };
            rising_or_falling = -1;
        } else if num > &last_number {
            if rising_or_falling == -1 {
                return false;
            };
            rising_or_falling = 1;
        } else {
            return false;
        }

        let diff = max(num, &last_number) - min(num, &last_number);

        if let 1..=3 = diff {
            println!("valid");
        } else {
            println!("invalid");
            return false;
        }

        last_number = *num;
    }
    return true;
}
