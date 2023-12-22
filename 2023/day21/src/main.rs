use std::{fs, collections::HashMap};

const TRANSFORM : [(i32, i32);4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn p1() {
    let mut res : u64 = 0;
    let file = fs::read_to_string("example").unwrap();
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
    let mut lanterefish : Vec<Vec<u64>> = vec![vec![0; map[0].len()]; map.len()];
    
    let deep = 6;
    let deep = 64;
    //let deep = 10;
    //let deep = 100;
    //let deep = 26501365;
    let h = map.len() as i32;
    let w = map[0].len() as i32;
    lanterefish[start.0 as usize][start.1 as usize] = 1;
    for i in 0..deep {
        let mut lanterefish2 : Vec<Vec<u64>> = vec![vec![0; map[0].len()]; map.len()];
        for (y, line) in lanterefish.iter_mut().enumerate() {
            for (x, b) in &mut line.iter_mut().enumerate() {
                if *b == 0 {
                    continue;
                }
                for t in TRANSFORM {
                    let mut fy = y as i32 + t.0;
                    let mut fx = x as i32 + t.1;
                    if fy < 0 {
                        fy = h-1;
                    } 
                    else if fx < 0 {
                        fx = w-1;
                    }
                    else if fy >= h {
                        fy = 0;
                    }
                    else if fx >= w {
                        fx = 0;
                    }
                    if map[(fy) as usize][(fx) as usize] == '#' {
                        continue;
                    }
                    lanterefish2[fy as usize][fx as usize] = 1;
                }
                lanterefish2[y as usize][x as usize] = 0;
            }
        }
        lanterefish = lanterefish2;
    }
    for v in lanterefish {
        println!("{:?}", v);
        res += v.iter().sum::<u64>();
    }
    println!("{}", res);
}



fn main() {
    p1();
}
