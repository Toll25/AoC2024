use std::fs::read_to_string;

pub fn run() {
    let garden = get_garden();

    let mut marked = vec![vec![false; garden[0].len()]; garden.len()];
    let mut looked_at = vec![vec![false; garden[0].len()]; garden.len()];
    let no_map = vec![vec![false; garden[0].len()]; garden.len()];
    let mut plots: Vec<(i32, i32)> = Vec::new();
    for x in 0..garden.len() {
        for y in 0..garden[0].len() {
            if !marked[x][y] {
                // start parsing whole plot

                let mut plants_to_check: Vec<(usize, usize)> = Vec::new();
                plants_to_check.push((x, y));
                let mut size_counter = 0;
                let mut peri_counter = 0;
                loop {
                    let plant = plants_to_check.pop().unwrap();
                    let adjescents =
                        get_adjescents(&garden, &no_map, plant, garden[plant.0][plant.1]);
                    let unvisited_adjescents =
                        get_adjescents(&garden, &marked, plant, garden[plant.0][plant.1]);
                    for plant in &unvisited_adjescents {
                        if !looked_at[plant.0][plant.1] {
                            plants_to_check.push(*plant);
                        }
                        looked_at[plant.0][plant.1] = true;
                    }
                    peri_counter += 4 - adjescents.len() as i32;
                    //println!("{:#?}", plant);
                    size_counter += 1;
                    marked[plant.0][plant.1] = true;
                    //dbg!(&plants_to_check);
                    if plants_to_check.is_empty() {
                        println!("Added plot: {}|{}", size_counter, peri_counter);
                        plots.push((size_counter, peri_counter));
                        break;
                    }
                }
            }
            //for x in 0..garden.len() {
            //    for y in 0..garden[0].len() {
            //        if looked_at[x][y] {
            //            print!("#")
            //        } else {
            //            print!(".");
            //        }
            //    }
            //    println!();
            //}
            //println!();
        }
    }

    let mut counter = 0;
    for plot in &plots {
        counter += plot.0 * plot.1;
    }
    //println!("Plots: {:#?}", plots);
    println!("Plotsum: {:#?}", counter);
    //for x in 0..garden.len() {
    //    for y in 0..garden[0].len() {
    //        print!("{}", garden[x][y]);
    //    }
    //    println!();
    //}
}

fn get_adjescents(
    garden: &Vec<Vec<char>>,
    visited: &Vec<Vec<bool>>,
    cord: (usize, usize),
    char: char,
) -> Vec<(usize, usize)> {
    let mut cords_to_check = vec![];

    if cord.0 + 1 < garden.len() {
        cords_to_check.push((cord.0 + 1, cord.1));
    }
    if cord.1 + 1 < garden.len() {
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
        if let Some(line) = garden.get(cord1.0) {
            if let Some(char1) = line.get(cord1.1) {
                //println!("comparing value {char1}");
                if !visited[cord1.0][cord1.1] {
                    if *char1 == char {
                        valids.push(cord1);
                    }
                }
            }
        }
    }
    valids
}
fn get_garden() -> Vec<Vec<char>> {
    let content = read_to_string("data.txt").unwrap();

    let mut garden: Vec<Vec<char>> = Vec::new();
    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut char_vec = Vec::new();
        for char in line.chars() {
            char_vec.push(char);
        }
        garden.push(char_vec);
    }
    garden
}
