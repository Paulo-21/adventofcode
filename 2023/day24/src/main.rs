use std::{fs, cmp::Ordering, process::exit, time::Instant};

//const MIN : f64 = 7.;
//const MAX : f64 = 27.;
const MIN : f64 = 200000000000000.;
const MAX : f64 = 400000000000000.;

fn p1() {
    //let file = fs::read_to_string("example").unwrap();
    let file = fs::read_to_string("input").unwrap();
    let mut res = 0;
    let mut all_coord = Vec::new();
    let mut all_vel = Vec::new();
    for line in file.lines() {
        let mut s  = line.split('@');
        let a = s.next().unwrap().split(", ").map(|str| str.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let b = s.next().unwrap().split(", ").map(|str| str.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let coord = [a[0], a[1], a[2]];
        let velocity = [b[0], b[1], b[2]];
        all_coord.push(coord);
        all_vel.push(velocity);
    }

    for (k, c) in all_coord.iter().enumerate() {
        for (i, cmp) in all_coord[k+1..].iter().enumerate() {
            let r = compute_intersec_pt(&c, &cmp, &all_vel[k], &all_vel[k+i+1]);
            if MIN < r.0 && r.0 < MAX && MIN < r.1 && r.1 < MAX {
                if !from_the_past(r, &c, &all_vel[k]) && !from_the_past(r, &cmp, &all_vel[k+i+1]) {
                    res+=1;
                }
            }
        }
    }
    println!("{}", res);
}

fn from_the_past(r : (f64,f64), coord : &[i64;3], velo : &[i64;3]) -> bool {
    if velo[0].is_negative() {
        if r.0 > coord[0] as f64 {
            return true;
        }
    }
    else {
        if r.0 < coord[0] as f64 {
            return true;
        }
    }
    if velo[1].is_negative() {
        if r.1 > coord[1] as f64 {
            return true;
        }
    }
    else {
        if r.1 < coord[1] as f64 {
            return true;
        }
    }
    false
}

fn compute_intersec_pt(coord1 :&[i64;3], coord2 :&[i64;3], velo1 :&[i64;3], velo2 :&[i64;3]) -> (f64, f64) {
    let a1 = velo1[1]as f64 / velo1[0] as f64;
    let a2 = velo2[1]as f64 / velo2[0] as f64;
    let b1 = coord1[1] as f64 - (a1*coord1[0] as f64);
    let b2 = coord2[1]as f64 - (a2 as f64*coord2[0] as f64);

    let x = (b1-b2 ) / (-a1 + a2);
    let y = a1 * x + b1;
    (x,y)
}

fn main() {
    p1();
}
