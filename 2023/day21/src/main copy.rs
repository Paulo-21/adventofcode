use std::{fs, collections::HashMap};

const TRANSFORM : [(i32, i32);4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn print_map(map : &Vec<Vec<char>>, reached : &Vec<Vec<bool>>) {
    for (i, v) in map.iter().enumerate() {
        for (k, c) in v.iter().enumerate() {
            if reached[i][k] {
                print!("O");
            }
            else {
                print!("{}", c);
            }
        }
        println!();
    }
}
fn p1() {
    let mut res = 0;
    //let file = fs::read_to_string("example").unwrap();
    let file = fs::read_to_string("input").unwrap();
    let mut map : Vec<Vec<char>> = Vec::new(); 
    let mut start = (0,0);
    for (i, line) in file.lines().enumerate() {
        if line.contains("S") {
            let x = line.chars().position(|c| c=='S').unwrap();
            start = (i as i32, x as i32);
        }
        map.push(line.chars().collect());
    }
    let mut reached : Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()]; 
    //let deep = 6;
    let deep = 64;
    let mut dp = HashMap::new();
    rec(&mut dp, &map , &mut reached, start, deep);

    for v in &reached {
        for b in v {
            if *b { res+= 1 }
        }
    }
    
    println!("{}", res);
    //print_map(&map, &reached);
}

fn rec(dp : &mut HashMap<((i32, i32), usize), bool> ,map : &Vec<Vec<char>>, reached : &mut Vec<Vec<bool>>, start : (i32, i32), deep : usize ) {
    if deep == 0 {
        reached[start.0 as usize][start.1 as usize] = true;
        return;
    }
    if let Some(x) = dp.get(&(start, deep)) {
        if *x {
            return;
        }
    }
    for t in TRANSFORM {
        if start.0+t.0 < 0 || start.1+t.1 < 0 || start.0+t.0 >= map.len() as i32 
            || start.1+t.1 >= map[0].len()as i32 || 
            map[(start.0+t.0) as usize][(start.1+t.1) as usize] == '#' {
            continue;
        }
        rec(dp, map, reached, (start.0+t.0, start.1+t.1), deep-1);
    }
    dp.insert((start, deep), true);
}

fn main() {
    p1();
}
