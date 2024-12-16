use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

#[derive(Debug, PartialEq, Clone, Eq, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn is_opposite(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Up, Self::Down)
                | (Self::Down, Self::Up)
                | (Self::Left, Self::Right)
                | (Self::Right, Self::Left)
        )
    }
}

fn print_maze(
    pos: &(usize, usize),
    start: &(usize, usize),
    end: &(usize, usize),
    maze: &Vec<Vec<bool>>,
) {
    for (index1, line) in maze.iter().enumerate() {
        for (index2, point) in line.iter().enumerate() {
            if !point {
                print!("#");
            } else if (index1, index2) == *start {
                print!("S");
            } else if (index1, index2) == *end {
                print!("E");
            } else if (index1, index2) == *pos {
                print!("P");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
fn get_neighbors(
    maze: &Vec<Vec<bool>>,
    pos: &(usize, usize),
    nodes: &Vec<(usize, usize)>,
) -> Vec<Edge> {
    let mut neighbors = Vec::new();
    let up = (pos.0 - 1, pos.1);
    let down = (pos.0 + 1, pos.1);
    let left = (pos.0, pos.1 - 1);
    let right = (pos.0, pos.1 + 1);
    // Up
    if maze[up.0][up.1] {
        neighbors.push(Edge {
            dir: Direction::Up,
            node: find_index(nodes, &up),
        });
    }
    // Down
    if maze[down.0][down.1] {
        neighbors.push(Edge {
            dir: Direction::Down,
            node: find_index(nodes, &down),
        });
    }
    // Left
    if maze[left.0][left.1] {
        neighbors.push(Edge {
            dir: Direction::Left,
            node: find_index(nodes, &left),
        });
    }
    // Right
    if maze[right.0][right.1] {
        neighbors.push(Edge {
            dir: Direction::Right,
            node: find_index(nodes, &right),
        });
    }
    neighbors
}

fn find_index(vec: &Vec<(usize, usize)>, find: &(usize, usize)) -> usize {
    for (index, node) in vec.iter().enumerate() {
        if node == find {
            return index;
        }
    }
    panic!("couldnt find");
}

#[derive(Debug)]
struct Edge {
    dir: Direction,
    node: usize,
}

fn main() {
    let (start, end, maze) = get_maze();
    //print_maze(&start, &start, &end, &maze);
    let mut nodes = Vec::new();
    let mut edge_map = Vec::new();
    let mut counter = 0;
    let mut start_index = 0;
    let mut end_index = 0;

    for (index1, line) in maze.iter().enumerate() {
        for (index2, value) in line.iter().enumerate() {
            if *value {
                nodes.push((index1, index2));
            }
        }
    }
    for (index1, line) in maze.iter().enumerate() {
        for (index2, value) in line.iter().enumerate() {
            if *value {
                let neighbors = get_neighbors(&maze, &(index1, index2), &nodes);
                if index1 == start.0 && index2 == start.1 {
                    start_index = counter;
                }
                if index1 == end.0 && index2 == end.1 {
                    end_index = counter;
                }
                edge_map.push(neighbors);
                counter += 1;
            }
        }
    }
    dbg!(edge_map.len());
    dbg!(counter);
    //dbg!(&edge_map);
    let short = shortest_path(&edge_map, start_index, end_index, Direction::Right);

    dbg!(short);
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: Node,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    position: usize,
    direction: Direction,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.position.cmp(&other.node.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn shortest_path(
    edge_map: &Vec<Vec<Edge>>,
    start: usize,
    goal: usize,
    start_dir: Direction,
) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<Node, usize> = HashMap::new();

    let mut heap = BinaryHeap::new();
    //    if (x, y, direction) in visited and visited[(x, y, direction)] <= dist:
    //        continue
    //
    //    visited[(x, y, direction)] = dist
    //
    //    for new_direction, turn_cost in turns[direction]:
    //        dx, dy = directions[new_direction]
    //        nx, ny = x + dx, y + dy
    //
    //        if 0 <= nx < len(grid) and 0 <= ny < len(grid[0]) and grid[nx][ny] != '#':
    //            new_cost = dist + turn_cost + 1
    //            if (nx, ny, new_direction) not in visited or visited[(nx, ny, new_direction)] > new_cost:
    //                heapq.heappush(heap, (new_cost, nx, ny, new_direction))
    dist.insert(
        Node {
            position: start,
            direction: start_dir,
        },
        0,
    );
    heap.push(State {
        cost: 0,
        node: Node {
            position: start,
            direction: start_dir,
        },
    });

    while let Some(State { cost, node }) = heap.pop() {
        println!("{}", node.position);
        if node.position == goal {
            println!("Goal: {cost}");
            return Some(cost);
        }
        if cost > dist[&node] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &edge_map[node.position] {
            let new_cost = cost + if edge.dir == node.direction { 1 } else { 1001 };
            let next = State {
                cost: new_cost,
                node: Node {
                    position: edge.node,
                    direction: edge.dir,
                },
            };

            // If so, add it to the frontier and continue
            if !dist.contains_key(&next.node) || next.cost < dist[&next.node] {
                //println!(
                //    "Relaxing node {} with cost {} (previous cost: {})",
                //    next.position, next.cost, dist[next.position]
                //);
                heap.push(next);
                // Relaxation, we have now found a better way
                let cost = &mut next.cost.clone();
                dist.entry(next.node).or_insert(*cost);
            }
        }
    }

    //dbg!(prev);

    // Goal not reachable
    None
}

fn get_maze() -> ((usize, usize), (usize, usize), Vec<Vec<bool>>) {
    let content = read_to_string("data.txt").unwrap();

    let mut maze = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (index1, line) in content.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut vec = Vec::new();
        for (index2, char) in line.chars().enumerate() {
            if char == '.' || char == 'S' || char == 'E' {
                vec.push(true);
                if char == 'S' {
                    start = (index1, index2);
                }
                if char == 'E' {
                    end = (index1, index2);
                }
            } else {
                vec.push(false);
            }
        }
        maze.push(vec);
    }
    (start, end, maze)
}
