use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

const SIZE: usize = 71;
fn main() {
    let mut coords = get_coords();

    let mut maze = vec![vec![false; SIZE]; SIZE];

    coords.reverse();
    for _index in 0..1024 {
        let cord = coords.pop().unwrap();
        maze[cord.0][cord.1] = true;
    }
    let path = simulate(&maze);

    while let Some(cord) = coords.pop() {
        maze[cord.0][cord.1] = true;
        if simulate(&maze).is_empty() {
            dbg!(cord);
            break;
        }
    }

    dbg!(path.len() - 1);
}

fn simulate(maze: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let source = (0, 0);
    let target = (SIZE - 1, SIZE - 1);

    if maze[source.0][source.1] || maze[target.0][target.1] {
        println!("Source or target is blocked!");
        return Vec::new();
    }

    let mut dist = HashMap::new();
    let mut prev: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(source, 0);
    heap.push(Reverse((0, source))); // Min-heap

    while let Some(Reverse((d, node))) = heap.pop() {
        if node == target {
            println!("Reached target with distance: {}", d);
            break;
        }

        for neighbor in get_neighbors(&maze, &node) {
            let new_dist = d + 1;
            if new_dist < *dist.get(&neighbor).unwrap_or(&i64::MAX) {
                dist.insert(neighbor, new_dist);
                prev.insert(neighbor, Some(node));
                heap.push(Reverse((new_dist, neighbor)));
            }
        }
    }
    let mut path = Vec::new();
    let mut current = target;

    while let Some(&Some(prev_node)) = prev.get(&current) {
        path.push(current);
        current = prev_node;
    }

    if current == source {
        path.push(source);
        path.reverse();
        println!("Shortest path: {:?}", path);
    } else {
        println!("No path found from source to target.");
    }
    path
}

fn get_neighbors(maze: &Vec<Vec<bool>>, cord: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    let rows = maze.len();
    let cols = maze[0].len();

    // Up
    if cord.0 > 0 {
        let up = (cord.0 - 1, cord.1);
        if !maze[up.0][up.1] {
            neighbors.push(up);
        }
    }

    // Down
    if cord.0 + 1 < rows {
        let down = (cord.0 + 1, cord.1);
        if !maze[down.0][down.1] {
            neighbors.push(down);
        }
    }

    // Left
    if cord.1 > 0 {
        let left = (cord.0, cord.1 - 1);
        if !maze[left.0][left.1] {
            neighbors.push(left);
        }
    }

    // Right
    if cord.1 + 1 < cols {
        let right = (cord.0, cord.1 + 1);
        if !maze[right.0][right.1] {
            neighbors.push(right);
        }
    }

    neighbors
}

fn get_coords() -> Vec<(usize, usize)> {
    let contents = read_to_string("data.txt").unwrap();
    let mut coords = Vec::new();
    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }
        let split: Vec<&str> = line.split(',').collect();
        coords.push((
            split[1].trim().parse::<usize>().unwrap(),
            split[0].trim().parse::<usize>().unwrap(),
        ));
    }
    coords
}
