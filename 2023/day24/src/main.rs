use std::{fs, time::Instant};
use z3::{Config, Context, Solver};
use z3::ast::{Real, Ast, Int};

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
            let r = compute_realersec_pt(&c, &cmp, &all_vel[k], &all_vel[k+i+1]);
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

fn compute_realersec_pt(coord1 :&[i64;3], coord2 :&[i64;3], velo1 :&[i64;3], velo2 :&[i64;3]) -> (f64, f64) {
    let a1 = velo1[1]as f64 / velo1[0] as f64;
    let a2 = velo2[1]as f64 / velo2[0] as f64;
    let b1 = coord1[1] as f64 - (a1*coord1[0] as f64);
    let b2 = coord2[1]as f64 - (a2 as f64*coord2[0] as f64);

    let x = (b1-b2 ) / (-a1 + a2);
    let y = a1 * x + b1;
    
    (x,y)
}

fn p2() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    let x = Real::new_const(&ctx, "x");
    let y = Real::new_const(&ctx, "y");
    let z = Real::new_const(&ctx, "z");
    let vx = Real::new_const(&ctx, "vx");
    let vy = Real::new_const(&ctx, "vy");
    let vz = Real::new_const(&ctx, "vz");
    for (i, line) in file.lines().enumerate() {
        let mut s  = line.split('@');
        let a = s.next().unwrap().split(", ").map(|str| str.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let b = s.next().unwrap().split(", ").map(|str| str.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let coord = [Real::from_int(&Int::from_i64(&ctx, a[0])), Real::from_int(&Int::from_i64(&ctx, a[1])), Real::from_int(&Int::from_i64(&ctx, a[2]))];
        let velocity = [Real::from_int(&Int::from_i64(&ctx, b[0])), Real::from_int(&Int::from_i64(&ctx, b[1])), Real::from_int(&Int::from_i64(&ctx, b[2]))];
        let mut t_name = String::from("t");
        t_name.push_str(i.to_string().as_str());
        //prRealln!("{}", t_name);
        let t = Real::new_const(&ctx, t_name);
        let a = Real::add(&ctx, &[&coord[0], &Real::mul(&ctx, &[&t, &velocity[0]]) ]);
        let b = Real::add(&ctx, &[&x, &Real::mul(&ctx, &[&t, &vx]) ]);
        //x + vx*t == t*Real(vel[0]) + Real(cord[0]);
        solver.assert(&a._eq(&b));
        let a = Real::add(&ctx, &[&coord[1], &Real::mul(&ctx, &[&t, &velocity[1]]) ]);
        let b = Real::add(&ctx, &[&y, &Real::mul(&ctx, &[&t, &vy]) ]);
        //x + vx*t == t*Real(vel[0]) + Real(cord[0]);
        solver.assert(&a._eq(&b));
        let a = Real::add(&ctx, &[&coord[2], &Real::mul(&ctx, &[&t, &velocity[2]]) ]);
        let b = Real::add(&ctx, &[&z, &Real::mul(&ctx, &[&t, &vz]) ]);
        //x + vx*t == t*Real(vel[0]) + Real(cord[0]);
        solver.assert(&a._eq(&b));
        //prRealln!("hello");
        if i == 2 {
            break;
        }
    }
    println!("Checking");
    println!("{:?}",solver.check());
    let m = solver.get_model().unwrap();
    println!("{:?}", m.eval(&Real::add(&ctx, &[&x, &y, &z]), false).unwrap());
}

fn main() {
    p1();
    p2();
}
