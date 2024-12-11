use std::fs::read_to_string;

fn main() {
    let trail = get_trail();
    let mut trailheads = Vec::new();
    for (index1, x) in trail.iter().enumerate() {
        for (index2, y) in x.iter().enumerate() {
            if *y == 0 {
                trailheads.push((index1, index2));
            }
        }
    }

    let mut counter = 0;
    for trailhead in trailheads {
        let mut moves_to_test: Vec<((usize, usize), usize)> = vec![(trailhead, 0)];
        let mut finisher_map: Vec<Vec<bool>> = vec![vec![false; trail[0].len()]; trail.len()];
        loop {
            let cord = moves_to_test.pop().unwrap();

            println!(
                "Checking coords {}|{} with value {}",
                cord.0 .0, cord.0 .1, cord.1
            );
            if cord.1 == 9
            // Part 1 toggle
            //&&  !finisher_map[cord.0 .0][cord.0 .1]
            {
                counter += 1;
                finisher_map[cord.0 .0][cord.0 .1] = true;
            }

            let new_moves = get_moves(&trail, cord.0, cord.1);

            moves_to_test.extend(new_moves.iter().map(|x| (*x, cord.1 + 1)));

            if moves_to_test.is_empty() {
                break;
            }
        }
    }
    dbg!(counter);
}

fn get_moves(trail: &Vec<Vec<usize>>, cord: (usize, usize), num: usize) -> Vec<(usize, usize)> {
    let mut cords_to_check = vec![];

    if cord.0 + 1 < trail.len() {
        cords_to_check.push((cord.0 + 1, cord.1));
    }
    if cord.1 + 1 < trail.len() {
        cords_to_check.push((cord.0, cord.1 + 1));
    }
    if let Some(new_cord) = cord.0.checked_sub(1) {
        cords_to_check.push((new_cord, cord.1));
    }
    if let Some(new_cord) = cord.1.checked_sub(1) {
        cords_to_check.push((cord.0, new_cord));
    }

    let mut valids = Vec::new();
    for cord1 in cords_to_check {
        if let Some(line) = trail.get(cord1.0) {
            if let Some(num1) = line.get(cord1.1) {
                println!("comparing value {num1}");
                if *num1 == num + 1 {
                    valids.push(cord1);
                }
            }
        }
    }
    valids
}

fn get_trail() -> Vec<Vec<usize>> {
    let mut content = read_to_string("test.txt").unwrap();

    let mut disk = Vec::new();
    let _ = content.pop();
    for line in content.split('\n') {
        disk.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }

    disk
}
