use std::fs;
fn day1() {
    let input = fs::read_to_string("input").unwrap();
    let s = input.lines();
    let mut res = 0;
    for line in s {
        
        for c in line.chars() {
            if c.is_numeric() {
                res += 10 * c.to_digit(10).unwrap();
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                res += c.to_digit(10).unwrap();
                break;
            }
        }
    }
    println!("{res}");
}
fn day2() {
    let number_str = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let input = fs::read_to_string("input").unwrap();
    let s = input.lines();
    let mut res = 0;
    for line in s {
        
        for (i,c) in line.chars().enumerate() {
            if c.is_numeric() {
                res += 10 * c.to_digit(10).unwrap();
                break;
            }
            else {
                let mut find = false;
                for n in number_str {
                    if n.len() + i >= line.len() {
                        continue;
                    }
                    let a = &line[i..i+n.len()];
                    if a == n {
                        find = true;
                        break;
                    }
                }
                if find {
                    break;
                }
            }
        }
        for (i,c) in line.chars().rev().enumerate() {
            if c.is_numeric() {
                res += c.to_digit(10).unwrap();
                break;
            }
            else {
                let mut find = 0;
                for (k, n) in number_str.iter().enumerate() {
                    if (i as i8) - (n.len() as i8) < 0 {
                        continue;
                    }
                    let a = &line[i-n.len()..i+1];
                    if *a == **n {
                        find = k;
                        break;
                    }
                }
                if find != 0 {
                    break;
                }
            }
        }
    }
    println!("{res}");
}
fn main() {
    //day1();
    day2();
}


