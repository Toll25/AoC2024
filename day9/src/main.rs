use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let disk = get_disk();
    let mut long_disk: Vec<Option<u32>> = Vec::new();
    let mut file_index: u32 = 0;
    let mut is_file = true;
    let mut file_sizes = HashMap::new();
    let mut file_indicis = HashMap::new();
    for (index, num) in disk.iter().enumerate() {
        if is_file {
            file_indicis.insert(file_index, long_disk.len());
            for _num in 0..*num {
                long_disk.push(Some(file_index));
            }
            file_sizes.insert(file_index, num);
            file_index += 1;
        } else {
            for _num in 0..*num {
                long_disk.push(None);
            }
        }
        is_file = !is_file;
    }
    //print_disk(&long_disk);

    //let long_disk_clone = long_disk.clone();

    // Part1
    //for (index1, el) in long_disk_clone.iter().enumerate() {
    //    if el.is_none() {
    //        // empty spot found
    //        //println!("empty found");
    //        for (index2, rev_el) in long_disk.iter().rev().enumerate() {
    //            if rev_el.is_some() {
    //                // element beginning from end found
    //                let index_rev = long_disk.len() - index2 - 1;
    //                //print!("{index1} ");
    //                //print!("{index_rev}");
    //                //println!("");
    //                long_disk.swap(index1, index_rev);
    //                //print_disk(&long_disk);
    //                break;
    //            }
    //        }
    //
    //        if validate(&long_disk) {
    //            break;
    //        }
    //    }
    //}
    ////print_disk(&long_disk);

    let mut keys: Vec<&u32> = file_sizes.keys().collect();
    keys.sort();
    keys.reverse();

    print_disk(&long_disk);
    for key in keys {
        let file_size = *(*file_sizes.get(key).unwrap()) as usize;
        let file_index = file_indicis.get(key).unwrap();

        //println!("File Size: {file_size}");
        //println!("File Index: {file_index}");
        //println!("Moving file: {key}");

        if let Some(space_index) = find_space(&long_disk, file_size) {
            //println!("Space Index: {space_index}");
            if space_index > *file_index {
                continue;
            }
            long_disk = move_many(&long_disk, *file_index, space_index, file_size);
        }
        //print_disk(&long_disk);
    }

    //dbg!(file_indicis);
    //dbg!(file_sizes);

    let mut counter = 0;
    for (index, el) in long_disk.iter().enumerate() {
        if let Some(num) = el {
            counter += index * *num as usize;
        }
    }
    println!("{counter}");
}

fn move_many(
    disk: &Vec<Option<u32>>,
    index1: usize,
    index2: usize,
    size: usize,
) -> Vec<Option<u32>> {
    let mut new_disk = disk.clone();
    //println!("Start index: {index2}");
    for num in 0..size {
        //println!("Swapping index {} and {}", index1 + num, index2 + num);
        new_disk.swap(index1 + num, index2 + num);
    }
    new_disk
}

fn find_space(disk: &Vec<Option<u32>>, size: usize) -> Option<usize> {
    let mut dot_counter = 0;
    let mut start_of_counter_index = 0;
    //println!("searching space with size: {size}");
    for (index, el) in disk.iter().enumerate() {
        if let Some(_num) = el {
            dot_counter = 0;
            start_of_counter_index = index;
        } else {
            dot_counter += 1;
            if size == dot_counter {
                return Some(start_of_counter_index + 1);
            }
        }
    }
    None
}
fn validate(disk: &Vec<Option<u32>>) -> bool {
    let mut encountered_dot = false;
    for el in disk {
        if let Some(_num) = el {
            if encountered_dot {
                return false;
            }
        } else {
            encountered_dot = true;
        }
    }
    true
}

fn print_disk(disk: &Vec<Option<u32>>) {
    for el in disk {
        if el.is_none() {
            print!(".");
        } else {
            print!("{}", el.unwrap());
        }
    }
    println!();
}

fn get_disk() -> Vec<u32> {
    let mut content = read_to_string("data.txt").unwrap();
    //println!("{content}");

    let _ = content.pop();
    let disk: Vec<u32> = content.chars().map(|x| x.to_digit(10).unwrap()).collect();
    disk
}
