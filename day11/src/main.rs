use std::collections::HashMap;

fn main() {
    let stones: Vec<u128> = vec![572_556, 22, 0, 528, 4_679_021, 1, 10725, 2790];
    //let stones: Vec<u128> = vec![125, 17];

    let mut map: HashMap<u128, u128> = HashMap::new();
    for stone in stones {
        insert_num(&mut map, stone, 1);
    }
    for current_blink in 0..75 {
        let mut new_stones = HashMap::new();

        for (stone, count) in &map {
            let count = *count;
            if *stone == 0 {
                insert_num(&mut new_stones, 1, count);
                continue;
            }
            let string = stone.to_string();
            if string.len() % 2 == 0 {
                let halves = string.split_at(string.len() / 2);
                insert_num(&mut new_stones, halves.0.parse::<u128>().unwrap(), count);
                insert_num(&mut new_stones, halves.1.parse::<u128>().unwrap(), count);
                continue;
            }
            insert_num(&mut new_stones, stone * 2024, count);
        }

        println!();
        println!("######### NEW MAP #########");
        for (value, count) in &map {
            println!("{value} x{count}");
        }
        println!();
        let mut counter = 0;
        for (_value, count) in map {
            counter += count;
        }
        println!("Count: {counter}");
        println!();
        println!("Blink No°: {current_blink}");
        map = new_stones;
    }

    let mut counter = 0;
    for (_value, count) in map {
        counter += count;
    }
    println!("{counter}");
}
fn insert_num(map: &mut HashMap<u128, u128>, num: u128, count: u128) {
    if let Some(count1) = map.get_mut(&num) {
        *count1 += count;
    } else {
        map.insert(num, count);
    }
}
