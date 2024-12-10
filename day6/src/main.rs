use std::fs::read_to_string;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let content = read_to_string("data.txt").unwrap();

    let mut maze: Vec<Vec<char>> = Vec::new();
    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut line_builder = Vec::new();
        for char in line.chars() {
            line_builder.push(char);
        }
        maze.push(line_builder);
    }

    let (_is_loop, visited) = simulate_maze(&maze);
    let mut count = 0;
    let mut count1 = 0;
    for x in visited.iter().enumerate() {
        for y in x.1.iter().enumerate() {
            //println!("new option checking: {} {}", x.0, y.0);
            if *y.1 {
                println!("Checking: {count}");
                count += 1;
                let mut new_maze = maze.clone();
                if new_maze[x.0][y.0] == '^' {
                    continue;
                }
                new_maze[x.0][y.0] = '#';
                let (is_loop, _) = simulate_maze(&new_maze);
                if is_loop {
                    count1 += 1;
                }
            }
        }
    }
    println!("Checked possilities: {count}");
    println!("Loops: {count1}");

    //let mut count = 0;
    //for x in visited {
    //    for y in x {
    //        if y {
    //            count += 1;
    //        }
    //    }
    //}
    //println!("{count}");
}

fn simulate_maze(maze: &Vec<Vec<char>>) -> (bool, Vec<Vec<bool>>) {
    let mut guard_direction = Direction::Up;
    let mut guard_coords = (0, 0);
    let mut visited: Vec<Vec<bool>> = vec![vec![false; maze[0].len()]; maze.len()];
    let mut is_loop = false;

    for x in maze.iter().enumerate() {
        for y in x.1.iter().enumerate() {
            if *y.1 == '^' {
                guard_coords = (x.0, y.0);

                visited[x.0][y.0] = true;
            }
        }
    }
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut steps = 0;
    loop {
        //look
        let mut occupied = false;
        let new_coords = match guard_direction {
            Direction::Up => {
                if guard_coords.0 > 0 {
                    (guard_coords.0 - 1, guard_coords.1)
                } else {
                    break;
                }
            }
            Direction::Down => (guard_coords.0 + 1, guard_coords.1),
            Direction::Left => {
                if guard_coords.1 > 0 {
                    (guard_coords.0, guard_coords.1 - 1)
                } else {
                    break;
                }
            }
            Direction::Right => (guard_coords.0, guard_coords.1 + 1),
        };

        if maze.get(new_coords.0).is_none() || maze[0].get(new_coords.1).is_none() {
            break;
        }
        let char = maze[new_coords.0][new_coords.1];
        match char {
            '#' => occupied = true,
            //'^' => {
            //    println!("loop found");
            //    is_loop = true;
            //    break;
            //}
            _ => {
                guard_coords = (new_coords.0, new_coords.1);
                visited[new_coords.0][new_coords.1] = true;
            }
        }
        //turn
        if occupied {
            match guard_direction {
                Direction::Up => guard_direction = Direction::Right,
                Direction::Down => guard_direction = Direction::Left,
                Direction::Left => guard_direction = Direction::Up,
                Direction::Right => guard_direction = Direction::Down,
            }
        }
        if steps > 10_000 {
            println!("took too long");
            is_loop = true;
            break;
        }
        //println!("{steps}");
        steps += 1;
    }
    (is_loop, visited)
}
