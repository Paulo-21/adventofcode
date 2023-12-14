use std::{fs, cmp::Ordering};//, time::Instant};
#[derive(Debug)]
enum TYPE {
    COL,
    ROW
}

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    //let file = fs::read_to_string("custum").unwrap();
    let mut pattern : Vec<Vec<char>> = Vec::new();
    let mut res = 0;
    for (k, line) in file.lines().enumerate() {
        if line.is_empty() {
            println!("{k}----------");
            print_pattern(&pattern);
            let res_analyse = analyse(&pattern).unwrap();  
            println!("{k} : {:?} {} {}", res_analyse.0, res_analyse.1.0+1, res_analyse.1.1+1);
            match res_analyse.0 {
                TYPE::COL => { res += res_analyse.1.0+1 },
                TYPE::ROW => { res += 100*(res_analyse.1.0+1) }
            }
            pattern = Vec::new();
            
            //break;
            continue;
        }
        pattern.push(line.chars().collect());
    }
    if !pattern.is_empty() {
        println!("----------");
        print_pattern(&pattern);
        if let Some(res_analyse) = analyse(&pattern) {
            match res_analyse.0 {
                TYPE::COL => { res += res_analyse.1.0+1 },
                TYPE::ROW => { res += 100*(res_analyse.1.0+1) }
            }
            println!(" : {:?} {} {}", res_analyse.0, res_analyse.1.0+1, res_analyse.1.1+1);
        }
    }
    println!("{}", res);
}


fn print_pattern(pattern : &Vec<Vec<char>>) {
    for v in pattern {
        println!("{}", v.iter().collect::<String>());
    }
}
fn check_for_perfect_match_row (pattern : &Vec<Vec<char>>, mut start : usize, mut end : usize) -> Option<(usize, usize)> {
    
    while start < end {
        let line1 = pattern.get(start).unwrap();
        let line2 = pattern.get(end).unwrap();
        if line1.cmp(line2) != Ordering::Equal {
            return None;
        }
        start +=1;
        end -=1;
    }
    Some((start-1, end+1))
}
fn check_for_perfect_match_col(pattern : &Vec<Vec<char>>, mut start : usize, mut end : usize) -> Option<(usize, usize)> {
    while start < end {
        if !is_equal_cols(pattern, start, end) {
            return None;
        }
        start +=1;
        end -=1;
    }
    Some((start-1, end+1))
}

fn analyse2_row(pattern : &Vec<Vec<char>>) -> Option<(TYPE, (usize, usize))> {
    for i in 1..pattern.len() {
        if let Some(x) = check_for_perfect_match_row(pattern, 0, pattern.len()-i) {
            return Some((TYPE::ROW, x));
        }
    }
    for i in 0..pattern.len()-1 {
        if let Some(x) = check_for_perfect_match_row(pattern, i, pattern.len()-1) {
            return Some((TYPE::ROW, x));
        }
    }
    None
}
fn analyse2_col(pattern : &Vec<Vec<char>>) -> Option<(TYPE, (usize, usize))> {
    for i in 1..pattern[0].len() {
        if let Some(x) = check_for_perfect_match_col(pattern, 0, pattern[0].len()-i) {
            return Some((TYPE::COL, x));
        }
    }
    for i in 0..pattern[0].len()-1 {
        if let Some(x) = check_for_perfect_match_col(pattern, i, pattern[0].len()-1) {
            return Some((TYPE::COL, x));
        }
    }
    None
}
fn is_equal_cols(pattern : &Vec<Vec<char>>, one :usize, two :usize) -> bool {
    for vec in pattern {
        if vec[one] != vec[two] {
            return false;
        }
    }
    true
}
fn analyse(pattern : &Vec<Vec<char>>) -> Option<(TYPE, (usize, usize))> {
    
    match analyse2_row(pattern) {
        Some(x) => {
            return Some(x);
        }, 
        None => {
            return analyse2_col(pattern);
        }
    }
}


fn main() {
    p1();
}
