use std::{fs, collections::HashSet, cmp::{min, max}};

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut all_lines  = Vec::new();
    let mut start = (0,0);
    let mut collision : HashSet<(i64,i64)> = HashSet::new();
    let mut line_n_one = true;
    for line in file.lines() {
        for action in line.split(",") {
            let direct = action.chars().next().unwrap();
            let length : i64 = action[1..].parse().unwrap();
            let mut next = start;
            match direct {
                'L' => next.1 -= length,
                'R' => next.1 += length,
                'D' => next.0 -= length,
                'U' => next.0 += length,
                _ => {}
            };
            if all_lines.len() >0 && !line_n_one {
                for past_line in all_lines[..all_lines.len()-1].iter() {
                    if let Some(col) = intersection((start, next), *past_line) {
                        collision.insert(col);
                    }
                }
            }
            if line_n_one {
                all_lines.push((start, next));
            }

            start = next;
        }
        line_n_one = false;
        start = (0,0);
    }
    let mut min = u64::MAX;
    for col in collision {
        let d = manathan((0,0), col);
        if d < min {
            min = d;
        }
    }
    println!("{}", min);
}
fn p2() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut all_lines: Vec<(((i64, i64), (i64, i64)), u64)>  = Vec::new();
    let mut start = (0,0);
    let mut line_n_one = true;
    let mut step = 0;
    let mut min = u64::MAX;
    for line in file.lines() {
        for action in line.split(",") {
            let direct = action.chars().next().unwrap();
            let length : i64 = action[1..].parse().unwrap();
            let mut next = start;
            match direct {
                'L' => next.1 -= length,
                'R' => next.1 += length,
                'D' => next.0 -= length,
                'U' => next.0 += length,
                _ => {}
            };
            if all_lines.len() > 0 && !line_n_one {
                for past_line in &all_lines {
                    if let Some(col) = intersection((start, next), (past_line).0) {
                        //println!("{}", (past_line).1 + step + get_dist(start, col)+ get_dist((past_line).0.0, col));
                        //println!("{:?} {:?} {:?} {:?}", ((start, next), step), past_line, get_dist(start, col), get_dist(past_line.0.0, col));
                        //println!("COL : {:?}", col);
                        if (past_line).1 + step + get_dist(start, col) + get_dist(past_line.0.0, col) < min {
                            min = (past_line).1 + step + get_dist(start, col) + get_dist(past_line.0.0, col);
                        }
                    }
                }
            }
            if line_n_one {
                all_lines.push(((start, next), step));
            }
            step += get_dist(start, next);
            start = next;
        }
        line_n_one = false;
        start = (0,0);
        step = 0;
    }

    
    println!("{}", min);
}
fn get_dist(one : (i64, i64), two : (i64,i64)) -> u64 {
    one.0.abs_diff(two.0) + one.1.abs_diff(two.1)
}
fn intersection(one : ((i64, i64),(i64,i64)), two : ((i64, i64),(i64,i64))) -> Option<(i64,i64)> {
    let x1 = one.0.1;
    let x2 = one.1.1;
    let y1 = one.0.0;
    let y2 = one.1.0;
    let x3 = two.0.1;
    let x4 = two.1.1;
    let y3 = two.0.0;
    let y4 = two.1.0;

    let a  = min(x1, x2) < x3 && x3 < max(x2, x1);
    let b  = min(x3, x4) < x1 && x1 < max(x4, x3);
    let c  = min(y3, y4) < y1 && y1 < max(y3,y4);
    let d  = min(y1, y2) < y3 && y3 < max(y2,y1);

    if a&&c { return  Some((y1,x3)); }
    else if b&&d { return Some((y3, x1)); }

    None
}
fn manathan(one : (i64, i64) , two: (i64,i64)) -> u64 {
    one.0.abs_diff(two.0) + one.1.abs_diff(two.1)
}

fn main() {
    p1();
    p2();
}
