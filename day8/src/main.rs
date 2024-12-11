use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

fn main() {
    // list of frequencies
    // get every pair of same frequency
    // get "distance" between them via diff of coords
    // apply diff to each element of pair
    // make seperate 2d array with antinodes
    // count antinodes
    let antennas = get_antennas();

    let mut freq_cords: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for x in antennas.iter().enumerate() {
        for y in x.1.iter().enumerate() {
            if *y.1 != '.' {
                if let Some(cords) = freq_cords.get_mut(y.1) {
                    cords.push((x.0, y.0));
                } else {
                    freq_cords.insert(*y.1, vec![(x.0, y.0)]);
                }
            }
        }
    }

    let mut anti = vec![vec![false; antennas[0].len()]; antennas.len()];

    for (_freq, cords) in freq_cords {
        dbg!(&cords);
        for combis in cords.iter().permutations(2) {
            // Part1
            //let diff = get_diff(*combis[0], *combis[1]);
            //let new_cord1 = ((*combis)[0].0 as isize) + (diff.0);
            //let new_cord2 = ((*combis)[0].1 as isize) + (diff.1);
            ////dbg!(new_cord1);
            ////dbg!(new_cord2);
            //if new_cord1 < 0 || new_cord1 >= anti.len() as isize {
            //    continue;
            //}
            //if new_cord2 < 0 || new_cord2 >= anti[0].len() as isize {
            //    continue;
            //}
            //anti[new_cord1 as usize][new_cord2 as usize] = true;
            ////dbg!(combis);
            // Part2
            let mut prev_cords = *combis[1];
            let mut cords = *combis[0];
            anti[cords.0][cords.1] = true;
            loop {
                let diff = get_diff(cords, prev_cords);
                let new_cord1 = (cords.0 as isize) + (diff.0);
                let new_cord2 = (cords.1 as isize) + (diff.1);
                //dbg!(new_cord1);
                //dbg!(new_cord2);
                if new_cord1 < 0 || new_cord1 >= anti.len() as isize {
                    break;
                }
                if new_cord2 < 0 || new_cord2 >= anti[0].len() as isize {
                    break;
                }
                anti[new_cord1 as usize][new_cord2 as usize] = true;
                prev_cords = cords;
                cords = (new_cord1 as usize, new_cord2 as usize);
                //dbg!(combis);
            }
        }
    }
    //dbg!(freq_cords);

    let mut count = 0;
    for x in anti {
        for y in x {
            if y {
                print!("#");
                count += 1;
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    for x in antennas {
        for y in x {
            print!("{y}");
        }
        println!();
    }
    println!("Num of anti antennas: {count}");
}

fn get_diff(cord1: (usize, usize), cord2: (usize, usize)) -> (isize, isize) {
    (
        cord1.0 as isize - cord2.0 as isize,
        cord1.1 as isize - cord2.1 as isize,
    )
}

fn get_antennas() -> Vec<Vec<char>> {
    let content = read_to_string("data.txt").unwrap();

    let mut antennas: Vec<Vec<char>> = Vec::new();
    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut char_vec = Vec::new();
        for char in line.chars() {
            char_vec.push(char);
        }
        antennas.push(char_vec);
    }
    antennas
}
