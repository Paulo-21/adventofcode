use std::{fs, cmp::Ordering};//, time::Instant};
use colored::Colorize;
#[derive(Debug, PartialEq)]
enum TYPE {
    COL,
    ROW
}

fn p2() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    //let file = fs::read_to_string("custom").unwrap();
    let mut pattern : Vec<Vec<char>> = Vec::new();
    let mut res = 0;
    for (k, line) in file.lines().enumerate() {
        if line.is_empty() {
            println!("{}", "NEXTNEXTNEXTNEXTNEXTNEXT".on_blue());
            let res_analyse = analyse(&pattern).unwrap();
            print_pattern(&pattern, &res_analyse);
            println!("{k} : {:?} {} {}", res_analyse.0, res_analyse.1.0+1, res_analyse.1.1+1);
            match res_analyse.0 {
                TYPE::COL => { res += res_analyse.1.0+1 },
                TYPE::ROW => { res += 100*(res_analyse.1.0+1) }
            }
            pattern = Vec::new();
            println!();
            println!();
            println!();
            println!();
            //break;
            continue;
        }
        pattern.push(line.chars().collect());
    }
    if !pattern.is_empty() {
        println!("{}", "NEXTNEXTNEXTNEXTNEXTNEXT".on_blue());
        if let Some(res_analyse) = analyse(&pattern) {
            print_pattern(&pattern, &res_analyse);
            match res_analyse.0 {
                TYPE::COL => { res += res_analyse.1.0+1 },
                TYPE::ROW => { res += 100*(res_analyse.1.0+1) }
            }
            println!(" : {:?} {} {}", res_analyse.0, res_analyse.1.0+1, res_analyse.1.1+1);
        }
    }
    println!("{}", res);
}


fn print_pattern(pattern : &Vec<Vec<char>>, res : &(TYPE, (usize, usize))) {
    for (_i, v) in pattern.iter().enumerate() {
        if res.0 == TYPE::ROW && _i ==  res.1.1 {
            print!("{}", vec!['-'; v.len()].iter().collect::<String>().red());
            println!();
        }
        for (k, c) in v.iter().enumerate() {
            if res.0 == TYPE::COL && k ==  res.1.1 {
                print!("{}", "|".red());
            }
            print!("{c}");
        }
        println!();
        //println!("{}", v.iter().collect::<String>());
    }
}
fn check_for_perfect_match_row (pattern : &Vec<Vec<char>>, mut start : usize, mut end : usize) -> Option<(usize, usize)> {
    let mut nb_diff_tot = 0;
    if start.abs_diff(end) == 2 {
        return None;
    }
    while start < end {
        nb_diff_tot += nb_diff_row(pattern, start, end);
        start +=1;
        end -=1;
    }
    
    if nb_diff_tot == 1 {
        return Some((start-1, end+1));
    }
    else {
        return None;
    }
}
fn check_for_perfect_match_col(pattern : &Vec<Vec<char>>, mut start : usize, mut end : usize) -> Option<(usize, usize)> {
    let mut nb_diff_tot = 0;
    if start.abs_diff(end) == 2 {
        return None;
    }
    while start < end {
        nb_diff_tot += nb_diff_cols(pattern, start, end);
        start +=1;
        end -=1;
    }
    if nb_diff_tot == 1 {
        return Some((start-1, end+1));
    }
    else {
        return None;
    }
}

fn analyse2_row(pattern : &Vec<Vec<char>>) -> Option<(TYPE, (usize, usize))> {

    for i in 1..pattern.len() {
        if let Some(x) = check_for_perfect_match_row(pattern, 0, pattern.len()-i) {
            return Some((TYPE::ROW, x));
        }
    }
    for i in 0..pattern.len()-1 {
        //println!("TEST");
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
fn nb_diff_cols(pattern : &Vec<Vec<char>>, one :usize, two :usize) -> usize {
    let mut nb_diff = 0;
    for vec in pattern {
        if vec[one] != vec[two] {
            nb_diff +=1;
        }
    }
    nb_diff
}
fn nb_diff_row(pattern : &Vec<Vec<char>>, one :usize, two :usize) -> usize {
    let mut nb_diff = 0;
    for (index, c) in pattern[one].iter().enumerate() {
        if *c != pattern[two][index] {
            nb_diff +=1;
            
        }
    }
    //println!("{} {} {}", nb_diff, one, two);
    nb_diff
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
    p2();
}
