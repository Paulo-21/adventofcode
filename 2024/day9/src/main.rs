use std::fs;

fn main() {
    //let input = fs::read_to_string("input").unwrap();
    let input = fs::read_to_string("exemple").unwrap();
    let mut res1 = 0;
    let mut occuped = Vec::new();
    let mut free = Vec::new();
    for (i,c) in input.chars().enumerate() {
        if i%2 == 0 { occuped.push((i/2, c.to_digit(10).unwrap())); }
        else { free.push((i/2, c.to_digit(10).unwrap())); }
    }
    let mut rev = occuped.clone();
    rev.reverse();
    for part in rev {
        
    }

}
