use std::{fs::read_to_string, i64};

use regex::Regex;

mod part1;
mod part2;

const X: usize = 101;
const Y: usize = 103;
//const X: usize = 11;
//const Y: usize = 7;

#[derive(Clone, Debug)]
pub struct Robot {
    px: usize,
    py: usize,
    vx: i64,
    vy: i64,
}

trait RobotVecExtensions {
    fn print_robots(&self);
    fn get_grid(&self) -> Vec<Vec<i32>>;
    fn run_simulation(&mut self);
}

impl RobotVecExtensions for Vec<Robot> {
    fn print_robots(&self) {
        let vec = self.get_grid();
        for x in vec {
            for y in x {
                if y == 0 {
                    print!(".");
                } else {
                    print!("{y}");
                }
            }
            println!();
        }
        println!();
    }
    fn get_grid(&self) -> Vec<Vec<i32>> {
        let mut vec = vec![vec![0; X]; Y];
        for robot in self {
            vec[robot.py][robot.px] += 1;
        }
        return vec;
    }
    fn run_simulation(&mut self) {
        for index in 0..self.len() {
            let robot = &mut self[index];
            let mut new_x = robot.px as i64 + robot.vx;
            let mut new_y = robot.py as i64 + robot.vy;
            new_x = new_x.rem_euclid(X as i64);
            new_y = new_y.rem_euclid(Y as i64);
            robot.px = new_x as usize;
            robot.py = new_y as usize;
        }
    }
}

fn main() {
    let mut robots = get_input();
    robots.print_robots();

    part2::run(robots.clone());
    for _i in 0..100 {
        robots.run_simulation();
        robots.print_robots();
    }
    robots.print_robots();
    let grid = robots.get_grid();
    let mut last_x = 0;
    let mut target_x = (grid[0].len() - 1) / 2;
    let mut counter1 = 0;
    let mut counter2 = 0;
    let mut counter3 = 0;
    let mut counter4 = 0;
    for i in last_x..target_x {
        let mut last_y = 0;
        let mut target_y = (grid.len() - 1) / 2;
        for j in last_y..target_y {
            counter1 += grid[j][i];
            //println!("{}{}", j, i);
        }
        last_y = target_y + 1;
        target_y = grid.len();
        for j in last_y..target_y {
            counter2 += grid[j][i];
            //println!("{}{}", j, i);
            //print!("{}", i);
        }
    }
    last_x = target_x + 1;
    target_x = grid[0].len();
    for i in last_x..target_x {
        let mut last_y = 0;
        let mut target_y = (grid.len() - 1) / 2;
        for j in last_y..target_y {
            counter3 += grid[j][i];
            //println!("{}{}", j, i);
        }
        last_y = target_y + 1;
        target_y = grid.len();
        for j in last_y..target_y {
            counter4 += grid[j][i];
            //println!("{}{}", j, i);
            //print!("{}", i);
        }
    }
    dbg!(counter1, counter2, counter3, counter4);
    println!("{}", counter1 * counter2 * counter3 * counter4)
    //part1::run(robots.clone());
    //part2::run(robots);
}

fn get_input() -> Vec<Robot> {
    let content = read_to_string("data.txt").unwrap();

    let re = Regex::new(
        r"p=(?<p1>[0-9]{1,3}),(?<p2>[0-9]{1,3}) v=(?<v1>-?[0-9]{1,3}),(?<v2>-?[0-9]{1,3})",
    )
    .unwrap();

    re.captures_iter(&content)
        .map(|x| {
            let px = x.name("p1").unwrap().as_str().parse::<usize>().unwrap();
            let py = x.name("p2").unwrap().as_str().parse::<usize>().unwrap();
            let vx = x.name("v1").unwrap().as_str().parse::<i64>().unwrap();
            let vy = x.name("v2").unwrap().as_str().parse::<i64>().unwrap();
            Robot { px, py, vx, vy }
        })
        .collect()
}
