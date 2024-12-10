use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    //dbg!(rules);
    let rules = get_rules();
    let updates = get_updates();

    let mut valid_updates: Vec<Vec<i32>> = Vec::new();
    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();
    for update in updates {
        if check_update(&rules, &update) {
            valid_updates.push(update);
        } else {
            invalid_updates.push(update);
        }
    }

    let mut new_valid_updates: Vec<Vec<i32>> = Vec::new();
    for update in invalid_updates {
        println!("checking new update");
        for combination in update
            .clone()
            .into_iter()
            .permutations(update.len())
            .unique()
        {
            if check_update(&rules, &combination) {
                new_valid_updates.push(combination.clone());
                println!("Found a match");
            }
        }
    }
    let mut inv_count = 0;
    for valid_update in new_valid_updates {
        inv_count += valid_update[valid_update.len() / 2];
    }

    println!("Invalid count: {inv_count}");

    let mut count = 0;
    for valid_update in valid_updates {
        count += valid_update[valid_update.len() / 2];
    }

    println!("{count}");

    //dbg!(valid_updates);
}

fn check_update(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    let mut visited_nums: Vec<i32> = Vec::new();
    for num in update {
        visited_nums.push(*num);
        let rules_for_num = rules.get(num).unwrap();
        for rule in rules_for_num {
            if update.contains(rule) && !visited_nums.contains(rule) {
                return false;
            }
        }
    }
    true
}

fn get_updates() -> Vec<Vec<i32>> {
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let updates_string = read_to_string("updates.txt").unwrap();

    for update_string in updates_string.split('\n') {
        if update_string.is_empty() {
            continue;
        }
        let update: Vec<i32> = update_string
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        updates.push(update);
    }

    dbg!(&updates.len());
    updates
}

fn get_rules() -> HashMap<i32, Vec<i32>> {
    let rules_string = read_to_string("rules.txt").unwrap();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule_string in rules_string.split('\n') {
        if rule_string.is_empty() {
            continue;
        }
        let split: Vec<String> = rule_string
            .split('|')
            .map(std::string::ToString::to_string)
            .collect();
        let el1 = split.first();
        let el2 = split.get(1);
        if el1.is_some() && el2.is_some() {
            let num1 = el1.unwrap().parse::<i32>().unwrap();
            let num2 = el2.unwrap().parse::<i32>().unwrap();

            if let Some(rules_for_num) = rules.get_mut(&num2) {
                rules_for_num.push(num1);
            } else {
                rules.insert(num2, vec![num1]);
            }
        } else {
            print!("INCORRECT RULE");
            continue;
        }
    }
    dbg!(&rules.len());
    rules
}
