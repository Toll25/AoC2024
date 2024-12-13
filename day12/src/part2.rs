use std::{fs::read_to_string, vec};

pub fn run() {
    let garden = get_garden();

    let mut marked = vec![vec![false; garden[0].len()]; garden.len()];
    let mut looked_at = vec![vec![false; garden[0].len()]; garden.len()];
    let no_map = vec![vec![false; garden[0].len()]; garden.len()];
    let mut plots: Vec<Vec<(usize, usize)>> = Vec::new();

    for x in 0..garden.len() {
        for y in 0..garden[0].len() {
            if !marked[x][y] {
                // start parsing whole plot

                let mut plants_to_check: Vec<(usize, usize)> = Vec::new();
                plants_to_check.push((x, y));
                let mut plot_cords = Vec::new();
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
                    //peri_counter += 4 - adjescents.len() as i32;
                    //println!("{:#?}", plant);
                    plot_cords.push(plant);
                    marked[plant.0][plant.1] = true;
                    //dbg!(&plants_to_check);
                    if plants_to_check.is_empty() {
                        plots.push(plot_cords);
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
    //dbg!(&plots);
    let mut plot_data: Vec<(usize, usize)> = Vec::new();
    for plot in plots {
        let mut peri_count = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        for cord in &plot {
            if max_x < cord.0 {
                max_x = cord.0;
            }
            if max_y < cord.1 {
                max_y = cord.1;
            }
        }
        let mut plot_garden: Vec<Vec<bool>> = vec![vec![false; max_y + 1]; max_x + 1];
        for cord in &plot {
            plot_garden[cord.0][cord.1] = true;
        }

        let mut hori_slices = plot_garden.clone();

        let mut vert_slices = Vec::new();
        for y in 0..(plot_garden[0].len()) {
            let mut vert_slice = Vec::new();
            for x in &plot_garden {
                vert_slice.push(x[y]);
            }
            vert_slices.push(vert_slice);
        }

        peri_count += get_edges(&hori_slices);
        hori_slices.reverse();
        peri_count += get_edges(&hori_slices);
        peri_count += get_edges(&vert_slices);
        vert_slices.reverse();
        peri_count += get_edges(&vert_slices);

        //for x in hori_slices {
        //    for y in x {
        //        if y {
        //            print!("#");
        //        } else {
        //            print!(".");
        //        }
        //    }
        //    println!();
        //}
        //println!();
        plot_data.push((peri_count, plot.len()));
    }

    let mut counter = 0;
    for plot in plot_data {
        counter += plot.0 * plot.1;
    }
    println!("Plots: {:#?}", counter);
    //println!("Plotsum: {:#?}", counter);
    //for x in 0..garden.len() {
    //    for y in 0..garden[0].len() {
    //        print!("{}", garden[x][y]);
    //    }
    //    println!();
    //}
}

fn get_edges(plot: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    let mut prev_slice: Vec<bool> = Vec::new();
    let mut plot = plot.clone();
    loop {
        let mut checked_slice;
        let slice;
        if prev_slice.is_empty() {
            slice = vec![false; plot[0].len()];
        } else {
            slice = plot.pop().unwrap();
        }
        checked_slice = Vec::new();
        //println!("comparing: ");
        //for y in &slice {
        //    if *y {
        //        print!("#");
        //    } else {
        //        print!(".");
        //    }
        //}
        //println!();
        //println!("with: ");
        //for y in &prev_slice {
        //    if *y {
        //        print!("#");
        //    } else {
        //        print!(".");
        //    }
        //}
        //println!();
        for i in 0..slice.len() {
            if slice[i] && !prev_slice[i] {
                checked_slice.push(true);
            } else {
                checked_slice.push(false);
            }
        }
        //println!("result: ");
        //for y in &checked_slice {
        //    if *y {
        //        print!("#");
        //    } else {
        //        print!(".");
        //    }
        //}
        //println!();
        let mut prev = false;
        for value in checked_slice {
            if !prev && value {
                count += 1;
            }
            prev = value;
        }
        //dbg!(&count);
        prev_slice = slice;
        if plot.is_empty() {
            break;
        }
    }
    count
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
