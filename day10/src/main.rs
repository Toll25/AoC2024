use std::fs::read_to_string;

fn main() {
    let trail = get_trail();
    dbg!(trail);
}
fn get_trail() -> Vec<Vec<usize>> {
    let mut content = read_to_string("data.txt").unwrap();

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
