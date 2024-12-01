use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut left_number = Vec::new();
    let mut right_number = Vec::new();
    let mut res = 0;
    for ligne in input.lines() {
        let split : Vec<&str> = ligne.split_whitespace().collect();
        left_number.push(split[0].parse::<u64>().unwrap());
        right_number.push(split[1].parse::<u64>().unwrap());
    }
    left_number.sort_unstable();
    right_number.sort_unstable();
    for index  in 0..left_number.len() {
        res += left_number[index].abs_diff(right_number[index]);
    }
    println!("{res}");
    let mut res = 0;
    for index  in 0..left_number.len() {
        res += left_number[index] * right_number.iter().fold(0, |acc, x| if *x == left_number[index] { acc + 1 } else { acc });
    }
    println!("{res}");
}
