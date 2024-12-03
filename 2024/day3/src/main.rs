use std::fs;
use regex::Regex;
fn main() {// 217909358 H
    let input = fs::read_to_string("input").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut res1 = 0;
    for caps in re.captures_iter(&input) {
        let x = caps[1].parse::<u32>().unwrap(); // Capturer X
        let y = caps[2].parse::<u32>().unwrap(); // Capturer Y
        res1 += x*y;
        //println!("Trouvé : X = {}, Y = {}", x, y);
    }
    println!("{}", res1);
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut res2 = 0;
    let mut do_mul = true;
    for caps in re.captures_iter(&input) {
        //println!("{:?}", caps);
        if caps[0].eq("do()") {
            do_mul = true
        }
        else if caps[0].eq("don't()") {
            do_mul = false;
        }
        else if do_mul {
            let x = caps[1].parse::<u32>().unwrap(); // Capturer X
            let y = caps[2].parse::<u32>().unwrap(); // Capturer Y
            res2 += x*y;
        }
    }
    println!("{}", res2);
}
