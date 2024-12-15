use core::f32;
use std::{collections::HashSet, fs};
use regex::Regex;

#[derive(Default, Debug, Clone, Copy)]
struct Robot {
    p : (i32,i32),
    v : (i32,i32),
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let xmax = 101;
    let ymax = 103;
    /*let input = fs::read_to_string("exemple").unwrap();
    let xmax = 11;
    let ymax = 7;*/
    let rep = Regex::new(r"p=(\d+),(\d+)").unwrap();
    let rev = Regex::new(r"v=(-?\d+),(-?\d+)").unwrap();
    let mut robots  = Vec::new();
    let step = 10000;
    for line in input.lines() {
        let (pos, velo) = line.split_once(" ").unwrap();
        let mut robot = Robot::default();
        if let Some(caps) = rep.captures(pos) {
            let x :i32 = caps[1].parse().unwrap();
            let y :i32 = caps[2].parse().unwrap();
            
            robot.p.0 = y;
            robot.p.1 = x;
        }
        if let Some(caps) = rev.captures(velo) {
            let x :i32 = caps[1].parse().unwrap();
            let y :i32 = caps[2].parse().unwrap();
            robot.v.0 = y;
            robot.v.1 = x;
        }
        robots.push(robot);

    }
    for sec in 1..=step {
        let mut s = HashSet::new();
        //println!("After {} s", sec);
        //print_map(&robots, ymax, xmax);
        for r in robots.iter_mut() {
            let mut ny = r.p.0 + r.v.0;
            let mut nx = r.p.1 + r.v.1;
            if ny < 0 {
                ny = ymax + ny;
            }
            else if ny >= ymax {
                ny = ny-ymax;
            }
            if nx < 0 {
                nx = xmax+nx;
            }
            else if nx >= xmax {
                nx = nx - xmax;
            }

            r.p.0 = ny;
            r.p.1 = nx;
            s.insert((ny,nx));
        }
        if sec == 100 {
            compute_safety(&robots, ymax, xmax);
        }
        if s.len() == 500 { 
            println!("SEC : {}", sec+1);
            let snap  =robots.clone(); 
            print_map(&snap, ymax, xmax);
        }
    }
    //print_map(&robots, ymax, xmax);
    //print_map(&snapchot, ymax, xmax);
    //compute_safety(&robots, ymax, xmax);
    //compute_safety(&snapchot, ymax, xmax);
}

fn compute_safety(robots : &Vec<Robot>, ymax : i32, xmax : i32) {
    let mut ul = 0;
    let mut ur = 0;
    let mut dl = 0;
    let mut dr = 0;
    let ymid = ymax/2;
    let xmid = xmax/2;
    for r in robots {
             if r.p.0 < ymid && r.p.1 < xmid { ur += 1; }
        else if r.p.0 < ymid && r.p.1 > xmid { ul += 1; }
        else if r.p.0 > ymid && r.p.1 > xmid { dr += 1; }
        else if r.p.0 > ymid && r.p.1 < xmid { dl += 1; }
    }
    println!("{} {} {} {}", ul,ur,dl,dr);
    println!("{}", ul*ur*dl*dr);
}

fn print_map(robots : &Vec<Robot>, ymax : i32, xmax : i32   ) {
    for y in 0..ymax {
        for x in 0..xmax {
            let r = how_many(robots, (y,x));
            if r > 0 { print!("{}", r); }
            else { print!("."); }
        }
        println!();
    }
    println!("---------------");
}
fn how_many(robots : &Vec<Robot>, pos : (i32,i32)) -> i32 {
    robots.iter().fold(0, |acc, x| if x.p.0 == pos.0 && x.p.1 == pos.1 { acc + 1} else { acc })
}
