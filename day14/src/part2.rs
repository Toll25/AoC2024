use std::{char::from_digit, fs::File, io::Write};

use crate::{Robot, RobotVecExtensions};

pub fn run(robots: Vec<Robot>) {
    let mut robots = robots;
    let mut full_string = String::new();
    for i in 0..10_000 {
        let vec = robots.get_grid();
        let mut string = format!("Counter: {i} \n").to_string();
        for x in &vec {
            for y in x {
                if *y == 0 {
                    string.push('.');
                } else {
                    string.push(from_digit(*y as u32, 10).unwrap());
                }
            }
            string.push('\n');
        }
        full_string += &string;
        robots.run_simulation();
    }
    let mut file = File::create("print.txt").unwrap();
    file.write_all(&full_string.into_bytes());
}
