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
                for n in number_str {
                    if n.len() + i >= line.len() {
                        continue;
                    }
                    let a = &line[i..i+n.len()];
                    if a == n {
                        break;
                    }
                }
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
fn main() {
    //day1();
    day2();
}


