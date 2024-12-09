use std::fs::read_to_string;

//use regex::Regex;

fn main() {
    let contents = read_to_string("data.txt").expect("Should have been able to read the file");

    let mut lines: Vec<Vec<char>> = contents
        .split('\n')
        .map(|split| split.chars().collect())
        .collect();
    let _ = lines.pop();
    let mut xmas_counter = 0;
    dbg!(&lines);
    for (height, line) in lines.iter().enumerate() {
        if line.len() < 3 {
            continue;
        }
        if height == 0 || height == lines.len() - 1 {
            continue;
        }
        for (width, char) in line.iter().enumerate() {
            if width == 0 || width == line.len() - 1 {
                continue;
            }
            if *char == 'A' {
                dbg!(line.len());
                dbg!(width);
                dbg!(height);
                let top_left = lines[height - 1][width - 1];
                let top_right = lines[height - 1][width + 1];
                let bottom_left = lines[height + 1][width - 1];
                let bottom_right = lines[height + 1][width + 1];

                if ((top_left == 'M')
                    && (top_right == 'M')
                    && (bottom_left == 'S')
                    && (bottom_right == 'S'))
                    || ((top_left == 'S')
                        && (top_right == 'S')
                        && (bottom_left == 'M')
                        && (bottom_right == 'M'))
                    || ((top_left == 'M')
                        && (top_right == 'S')
                        && (bottom_left == 'M')
                        && (bottom_right == 'S'))
                    || ((top_left == 'S')
                        && (top_right == 'M')
                        && (bottom_left == 'S')
                        && (bottom_right == 'M'))
                {
                    xmas_counter += 1;
                }
            }
        }
    }
    println!("{xmas_counter}");
}

//fn main() {
//    let mut lines_to_check: Vec<String> = Vec::new();
//
//    let contents = read_to_string("data.txt").expect("Should have been able to read the file");
//
//    // Horizontal
//    let lines: Vec<String> = contents
//        .split('\n')
//        .map(std::string::ToString::to_string)
//        .collect();
//    lines_to_check.append(&mut lines.clone());
//    let _ = lines_to_check.pop();
//
//    // Vertical
//    let mut vert_lines: Vec<Vec<char>> = vec![Vec::new(); lines_to_check.len()];
//    for line in &lines_to_check {
//        for (line_index, cha) in line.chars().enumerate() {
//            vert_lines[line_index].push(cha);
//        }
//    }
//    for line in vert_lines {
//        let string: String = line.into_iter().collect();
//        //println!("{string}");
//        lines_to_check.push(string);
//    }
//
//    // Diagonal Left
//    let mut start_points: Vec<(usize, usize)> = Vec::new();
//
//    for (height, _line) in lines.iter().enumerate() {
//        start_points.push((height, 0));
//    }
//    //let _ = start_points.pop();
//    for (width, _char) in lines.first().unwrap().chars().enumerate() {
//        if width == 0 {
//            continue;
//        }
//        start_points.push((0, width));
//    }
//    //let _ = start_points.pop();
//
//    for (start_height, start_width) in start_points {
//        let mut string = String::new();
//
//        let mut height = start_height;
//        let mut width = start_width;
//        while let Some(line) = lines.get(height) {
//            if let Some(char) = line.chars().nth(width) {
//                string.push(char);
//            }
//            height += 1;
//            width += 1;
//        }
//
//        lines_to_check.push(string);
//    }
//
//    // Diagonal Right
//    let mut start_points: Vec<(usize, usize)> = Vec::new();
//
//    for (height, line) in lines.iter().enumerate() {
//        start_points.push((height, line.len()));
//    }
//    //let _ = start_points.pop();
//    for (width, _char) in lines.first().unwrap().chars().enumerate() {
//        if width == 0 {
//            continue;
//        }
//        start_points.push((0, width));
//    }
//    //let _ = start_points.pop();
//
//    for (start_height, start_width) in start_points {
//        let mut string = String::new();
//
//        let mut height = start_height;
//        let mut width = start_width;
//        while let Some(line) = lines.get(height) {
//            if let Some(char) = line.chars().nth(width) {
//                string.push(char);
//            }
//            height += 1;
//            if width > 0 {
//                width -= 1;
//            } else {
//                break;
//            }
//        }
//
//        lines_to_check.push(string);
//    }
//
//    let regex = Regex::new(r"XMAS").unwrap();
//    let mut xmas_num = 0;
//    for line in &lines_to_check {
//        let reverse_line = line.chars().rev().collect::<String>();
//        xmas_num += regex.captures_iter(line).count();
//        xmas_num += regex.captures_iter(&reverse_line).count();
//        println!("{line}");
//    }
//    println!("Lines To check {}", lines_to_check.len());
//    println!("XMASes {xmas_num}");
//}
