use std::fs;
#[allow(unused_imports)]
use colored::Colorize;
use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);
const TRANSFORM : [(i32, i32) ;4] = [(0,1),(0,-1),(1,0),(-1,0)];

impl Pos {
  fn distance(&self, other: &Pos) -> u32 {
    (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
  }

    fn successors(&self, map : &Vec<Vec<u32>>) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        let mut succ = Vec::with_capacity(4);
        
        for t in TRANSFORM {
            let s = Pos(x+t.0, y+t.1);
            if  s.1 < 0 || s.1 >= map.len() as i32 || s.0 < 0 || s.0 >= map[0].len()as i32 {
                continue;
            }
            let cost = map[s.1 as usize][s.0 as usize];
            succ.push((s, cost));
        }
        succ
    }
}

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut map : Vec<Vec<u32>> = Vec::new();
    for line in file.lines() {
        map.push(line.chars().map(|c|c as u32 -48).collect());
    }
    //let start = (0usize,1usize);
    let goal :Pos = Pos((map.len()-1 ) as i32,(map[0].len()-1) as i32);
    
    let result = astar(&Pos(0, 0), |p| p.successors(&map), |p| p.distance(&goal),
                    |p| *p == goal);
    //assert_eq!(result.expect("no path found").1, 4);
    let path  = result.unwrap().0;
    let mut res = 0;
    /*for (i , p) in map.iter().enumerate() {
        for (k, v) in p.iter().enumerate() {
            if path.contains(&Pos(k as i32, i as i32)) {
                /*println!();
                println!(" {} {} ", k, i);*/
                print!("{}", v.to_string().green());
            }
            else {
                print!("{}", v.to_string());
            }
        }
        println!();
    }*/
    for c in path {
        res += map[c.1 as usize][c.0 as usize];
    }
    println!("{}", res-map[0][0]);
}
fn extend_map(map : &mut Vec<Vec<u32>>) {
    for line in map.iter_mut() {
        let mut line_save = line.clone();
        for _i in 0..4 {
            for n in line_save.iter_mut() {
                *n = (*n)%9 +1;
            }
            line.extend(line_save.iter());
        }
    }
    let mut maps_line_saver = map.clone();
    for _i in 0..4 {
        for line in maps_line_saver.iter_mut() {
            for n in line {
                *n = (*n)%9 +1;
            }
        }
        for line in maps_line_saver.iter() {
            map.push(line.clone());
        }
    }
}

fn p2() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut map : Vec<Vec<u32>> = Vec::new();
    for line in file.lines() {
        map.push(line.chars().map(|c|c as u32 -48).collect());
    }
    extend_map(&mut map);
    //let start = (0usize,1usize);
    let goal :Pos = Pos((map.len()-1 ) as i32,(map[0].len()-1) as i32);
    
    let result = astar(&Pos(0, 0), |p| p.successors(&map), |p| p.distance(&goal),
                    |p| *p == goal);
    //assert_eq!(result.expect("no path found").1, 4);
    let path  = result.unwrap().0;
    //println!("{:?}", path);
    let mut res = 0;
    /*for (i , p) in map.iter().enumerate() {
        for (k, v) in p.iter().enumerate() {
            if path.contains(&Pos(k as i32, i as i32)) {
                print!("{}", v.to_string().green());
            }
            else {
                print!("{}", v.to_string());
            }
        }
        println!();
    }*/
    for c in path {
        res += map[c.1 as usize][c.0 as usize];
    }
    println!("{}", res-map[0][0]);
}

fn main() {
    p1();
    p2();
}
