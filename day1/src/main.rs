use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("data.txt")
        .unwrap()
        .lines()
        .map(String::from) // make each slice into a string
        .collect();

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in lines {
        let mut split = line.split_whitespace();
        let number1 = split.next().unwrap().parse::<i32>().unwrap();
        let number2 = split.last().unwrap().parse::<i32>().unwrap();

        list1.push(number1);
        list2.push(number2);
        //number_pairs.insert(number1, number2);
    }
    list1.sort_unstable();
    list2.sort_unstable();

    print!("Total Difference: {}", solve1(list1.clone(), list2.clone()));

    let mut result = 0;
    for num in list1 {
        let count: i32 = list2
            .iter()
            .filter(|&n| *n == num)
            .count()
            .try_into()
            .unwrap();
        result += num * count;
    }
    println!("Result: {}", result);
}

fn solve1(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    let mut difference = 0;
    for (pos, num1) in list1.iter().enumerate() {
        let num2 = list2[pos];
        //println!("Number 1: {num1}");
        //println!("Number 2: {num2}");
        if num1 < &num2 {
            difference += num2 - num1;
        } else {
            difference += num1 - num2;
        }
    }
    difference
}
