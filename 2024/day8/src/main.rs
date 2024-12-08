use std::{collections::{HashMap, HashSet}, fs};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    let mut antena: HashMap<char, Vec<(i32,i32)>> = HashMap::new();
    let mut antinodes_p1: HashSet<(i32,i32)> = HashSet::new();
    let mut antinodes_p2: HashSet<(i32,i32)> = HashSet::new();

    let mut imax = 0;
    let mut jmax = 0;
    for (i,line) in input.lines().enumerate() {
        jmax = line.len() as i32;
        for (j,c) in line.chars().enumerate() {
            if c != '.' {
                if let Some(v) = antena.get_mut(&c) {
                    v.push((i as i32,j as i32));
                }
                else {
                    antena.insert(c, Vec::from([(i as i32,j as i32)]));
                }
            }
        }
        imax+=1;
    }
    let mut proced: HashSet<(i32,i32)> = HashSet::new();
    for (_, list) in antena.iter() {
        for pos1 in list {
            for pos2 in list {
                if *pos1 == *pos2 { break; }
                if proced.contains(pos2) {
                    let mut i = 1;
                    let dist = compute_dist(*pos1, *pos2);
                    antinodes_p2.insert(*pos1);
                    antinodes_p2.insert(*pos2);
                    loop {
                        let new_dist = (dist.0*i, dist.1*i);
                        let antinode1 = sub_tup(*pos1, new_dist);
                        let antinode2 = add_tup(*pos2, new_dist);
                        let mut out1 = true;
                        let mut out2 = true;
                        if !is_outside(antinode1, imax, jmax) {
                            if i == 1 {
                                antinodes_p1.insert(antinode1);
                            }
                            antinodes_p2.insert(antinode1);
                            out1 = false;
                        }
                        if !is_outside(antinode2, imax, jmax) {
                            if i == 1 { antinodes_p1.insert(antinode2); }
                            antinodes_p2.insert(antinode2);
                            out2 = false;
                        }
                        if out1 && out2 {
                            break;
                        }
                        i+=1;
                    }
                }
            }
            proced.insert(*pos1);
        }
    }
    /*for (i,line) in input.lines().enumerate() {
        for (j,c) in line.chars().enumerate() {
            if antinodes.contains(&(i as i32,j as i32)) {
                print!("#");
            }
            else if proced.contains(&(i as i32,j as i32)){
                print!("{}", c.green());
            }
            else if c == '.' {
                print!("{c}");
            }
        }
        println!();
    }*/
    println!("{}", antinodes_p1.len());//411 too Hight
    println!("{}", antinodes_p2.len());//411 too Hight
}
fn sub_tup(one : (i32,i32), dist : (i32,i32)) -> (i32,i32) {
    (one.0 - dist.0, one.1-dist.1)
}
fn add_tup(one : (i32,i32), dist : (i32,i32)) -> (i32,i32) {
    (one.0 + dist.0, one.1+dist.1)
}
fn compute_dist(pos1 : (i32,i32), pos2 : (i32,i32)) -> (i32,i32) {
    (pos2.0 - pos1.0, pos2.1-pos1.1)
}
fn is_outside(node : (i32,i32), imax:i32, jmax:i32) -> bool {
    node.0 < 0  || node.0 >= imax || node.1 < 0  || node.1 >= jmax
}