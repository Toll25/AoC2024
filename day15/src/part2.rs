use std::fs::read_to_string;

#[derive(Debug)]
struct Warehouse {
    grid: Vec<Vec<char>>,
    moves: Vec<char>,
}

trait GridExtensions {
    fn print(&self);
    fn get_robot(&self) -> (usize, usize);
    fn move_entity(&mut self, cords: (usize, usize), direction: char) -> bool;
    fn get_boxes(&self) -> Vec<(usize, usize)>;
}

impl GridExtensions for Vec<Vec<char>> {
    fn print(&self) {
        for line in self {
            for char in line {
                print!("{char}");
            }
            println!();
        }
        println!();
    }
    fn get_robot(&self) -> (usize, usize) {
        for (index, line) in self.iter().enumerate() {
            for (index1, char) in line.iter().enumerate() {
                if *char == '@' {
                    return (index, index1);
                }
            }
        }
        (0, 0)
    }
    fn move_entity(&mut self, cords: (usize, usize), direction: char) -> bool {
        let entity = self[cords.0][cords.1];
        match entity {
            '#' => {
                println!("Tried to move wall");
                return false;
            }
            'O' => {
                println!("Moving box");
            }
            '@' => {
                println!("Moving robot");
            }
            _ => {
                println!("Unknown entity");
            }
        }
        let new_cords = match direction {
            '^' => (cords.0 - 1, cords.1),
            'v' => (cords.0 + 1, cords.1),
            '<' => (cords.0, cords.1 - 1),
            '>' => (cords.0, cords.1 + 1),
            _ => return false,
        };

        let new_entity = self[new_cords.0][new_cords.1];
        let mut status = true;

        match new_entity {
            '#' => {
                println!("Tried to move into wall");
                return false;
            }
            'O' => {
                status = self.move_entity(new_cords, direction);
                println!("Moving box");
            }
            '@' => {
                println!("how");
            }
            '.' => {
                //self[new_cords.0][new_cords.1] = entity;
                //self[cords.0][cords.1] = '.';
            }
            _ => {
                println!("Unknown entity");
            }
        }
        if status {
            self[new_cords.0][new_cords.1] = entity;
            self[cords.0][cords.1] = '.';
            return true;
        }
        false
    }

    fn get_boxes(&self) -> Vec<(usize, usize)> {
        let mut cords = Vec::new();
        for (index, line) in self.iter().enumerate() {
            for (index1, char) in line.iter().enumerate() {
                if *char == 'O' {
                    cords.push((index, index1));
                }
            }
        }
        cords
    }
}
pub fn run() {
    let mut warehouse = get_warehouse();
    warehouse.grid.print();
    loop {
        let direction = warehouse.moves.pop().unwrap();
        warehouse
            .grid
            .move_entity(warehouse.grid.get_robot(), direction);
        if warehouse.moves.is_empty() {
            break;
        }
        println!("Direction: {direction}");
        warehouse.grid.print();
    }

    let mut counter = 0;
    for cord in warehouse.grid.get_boxes() {
        counter += 100 * cord.0 + cord.1;
    }
    dbg!(counter);
}

fn get_warehouse() -> Warehouse {
    let content = read_to_string("data.txt").unwrap();
    let split: Vec<&str> = content.split("\n\n").collect();
    let grid_string = split[0];
    let moves_string = split[1];

    let mut grid = Vec::new();

    for line in grid_string.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut chars = Vec::new();
        for char in line.chars() {
            chars.push(char);
        }
        grid.push(chars);
    }

    let mut moves = Vec::new();

    for char in moves_string.chars() {
        if char == '\n' {
            continue;
        }
        moves.push(char);
    }
    moves.reverse();

    Warehouse { grid, moves }
}
