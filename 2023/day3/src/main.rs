use core::num;
use std::{fs, process::exit};
use std::collections::BTreeMap;
struct R {
    nb : u8,
    res : u32,
}
fn find_number(
    file : &String, 
    start_index : usize
) -> (usize, usize) {
    let file = file.as_str();
    let mut find = false;
    let mut find_non_numric = false;
    let mut numeric_index = 0;
    let mut non_numeric_index = file.len();
    let mut iter = file.char_indices();
    for (i, c) in iter {
        //println!("{}", start_index);
        if i < start_index {
            continue;
        }
        if c.is_digit(10) && !find {
            find = true;
            numeric_index = i;
        }
        if !c.is_digit(10) && find {
            non_numeric_index = i;
            break;
        }
    }
    
    (numeric_index,non_numeric_index)
}
fn is_adjacent(gear_pos : &mut BTreeMap<usize, R> ,file : &str, start: usize, end : usize, max_col : usize) -> bool {
    println!("IS adj");
    let value = file[start..end].parse::<usize>().unwrap();
    match file.chars().nth(start-1) {
        Some(c) => {
            println!("s  : {c}");
            if c == '*' {
                match gear_pos.get_mut(&(start-1)) {
                    Some(x) => {
                        x.nb +=1;
                        x.res *=value as u32;
                    },
                    None => { gear_pos.insert(start-1, R { nb: 1, res: value as u32 });}
                }
                return true;
            }
            if c != '.' && c != '\n' && c != '\r' {
                return true;
            }
        },
        None => {}
    }
    match file.chars().nth(start-1 - max_col) {
        Some(c) => {
            println!("s - :{c}");
            if c == '*' {
                match gear_pos.get_mut(&(start-1 - max_col)) {
                    Some(x) => {
                        x.nb +=1;
                        x.res *=value as u32;
                    },
                    None => { gear_pos.insert(start-1 - max_col, R { nb: 1, res: value as u32 });}
                }
                return true;
            }
            if c != '.' && c != '\n' && c != '\r' {
                return true;
            }
        },
        None => {}
    }
    match file.chars().nth(start-1 + max_col) {
        Some(c) => {
            println!("s + : {c}");
            if c == '*' {
                match gear_pos.get_mut(&(start-1 + max_col)) {
                    Some(x) => {
                        x.nb +=1;
                        x.res *=value as u32;
                    },
                    None => { gear_pos.insert((start-1 + max_col), R { nb: 1, res: value as u32 });}
                }
                return true;
            }
            if c != '.' && c != '\n' && c != '\r' {
                return true;
            }
        },
        None => {}
    }
    for k in start..end {
        match file.chars().nth(k - max_col) {
            Some(c) => {
                println!(" se - :{c}n");
                if c == '*' {
                    match gear_pos.get_mut(&(k - max_col)) {
                        Some(x) => {
                            x.nb +=1;
                            x.res *=value as u32;
                        },
                        None => { gear_pos.insert((k - max_col), R { nb: 1, res: value as u32 });}
                    }
                    return true;
                }
                if c != '.' && c != '\n' && c != '\r'{
                    return true;
                }
            },
            None => {}
        }
        match file.chars().nth(k + max_col) {
            Some(c) => {
                println!("se + : {c}");
                if c == '*' {
                    match gear_pos.get_mut(&(k + max_col)) {
                        Some(x) => {
                            x.nb +=1;
                            x.res *=value as u32;
                        },
                        None => { gear_pos.insert((k + max_col), R { nb: 1, res: value as u32 });}
                    }
                    return true;
                }
                if c != '.' && c != '\n' && c != '\r' {
                    return true;
                }
            },
            None => {}
        }
    }
        match file.chars().nth(end) {
        Some(c) => {
            println!("e {c}");
            if c == '*' {
                match gear_pos.get_mut(&end) {
                    Some(x) => {
                        x.nb +=1;
                        x.res *=value as u32;
                    },
                    None => { gear_pos.insert(end, R { nb: 1, res: value as u32 });}
                }
                return true;
            }
            if c != '.' && c != '\n' && c != '\r' {
                return true;
            }
        },
        None => {}
    }
    match file.chars().nth(end - max_col) {
        Some(c) => {
            println!("e -{c}");
            if c == '*' {
                match gear_pos.get_mut(&(end - max_col)) {
                    Some(x) => {
                        x.nb +=1;
                        x.res *=value as u32;
                    },
                    None => { gear_pos.insert((end - max_col), R { nb: 1, res: value as u32 });}
                }
                return true;
            }
            if c != '.' && c != '\n' && c != '\r' {
                return true;
            }
        },
        None => {}
    }
    match file.chars().nth(end + max_col) {
        Some(c) => {
            println!("e + {c}");
            if c == '*' {
                match gear_pos.get_mut(&(end + max_col)) {
                    Some(x) => {
                        x.nb +=1;
                        x.res *=value as u32;
                    },
                    None => { gear_pos.insert((end + max_col), R { nb: 1, res: value as u32 });}
                }
                return true;
            }
            if c != '.' && c != '\n' && c != '\r' {
                return true;
            }
        },
        None => {}
    }
    false
}
fn p1() {
    let mut fi = String::from("Helloe 
    dezdzdz");
    fi.retain(|c| c != '\n' && c != '\r');
    println!("{}", fi);
    let mut b = BTreeMap::new();
    //exit(0);
    let mut file = fs::read_to_string("input").unwrap();
    let (_max_ligne, max_col) = nb_ligne_and_col(&file);
    file.retain(|c| c != '\n' && c != '\r');
    let mut res = 0;
    let mut not_finish = true;
    
    let mut start_index = 0;
    while not_finish {
        let (st , end) = find_number(&file, start_index);
        let number = file.as_str();
        let n = number[st..end].parse::<u32>().unwrap();
        start_index = end;

        println!("{}", n);
        if is_adjacent(&mut b, file.as_str(), st, end, max_col) {
            println!("YES : {n}");
            res += n;
        }
        if end == file.len() {
            not_finish = false;
        }
        //break;
    }
    println!("{res}");
}
fn p2() {
    let mut gear_pos = BTreeMap::new();
    let mut file = fs::read_to_string("input").unwrap();
    let (_max_ligne, max_col) = nb_ligne_and_col(&file);
    file.retain(|c| c != '\n' && c != '\r');
    let mut res = 0;
    let mut not_finish = true;
    
    let mut start_index = 0;
    while not_finish {
        let (st , end) = find_number(&file, start_index);
        let number = file.as_str();
        let n = number[st..end].parse::<u32>().unwrap();
        start_index = end;

        println!("{}", n);
        if is_adjacent(&mut gear_pos, file.as_str(), st, end, max_col) {
            
            
        }
        if end == file.len() {
            not_finish = false;
        }
        //break;
    }
    for (i, node) in gear_pos {
        //println!("{:?} : ", gear_pos.get(&i).unwrap());
        println!("{i}");
        if node.nb == 2 {
            res += node.res;
        }
    }
    println!("{res}");
}

fn nb_ligne_and_col(file : &String) -> (usize, usize) {
    let mut nl = 0;
    let mut nc = 0;
    for l in file.lines() {
        if nl == 0 {
            nc = l.len();
        }
        nl+=1;
    }
    (nl,nc)
}
fn main() {
    //p1();
    p2();
}
