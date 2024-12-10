use std::{collections::HashSet, fs};

const DIR : [(i32,i32);4] = [(-1,0), (1,0), (0, -1), (0,1)];

fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        map.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
    }
    let mut score1 = 0;
    let mut score2 = 0;
    for (i, line) in map.iter().enumerate() {
        for (j,n) in line.iter().enumerate() {
            if *n == 0 {
                let mut already = HashSet::new();
                score1 += count_rando_path(&map, &mut already, (i as i32, j as i32));
                score2 += count_rando_path_p2(&map, (i as i32, j as i32));
            }
        }
    }
    println!("{}", score1);
    println!("{}", score2);
}
fn count_rando_path(map : &Vec<Vec<u8>>, already : &mut HashSet<(i32,i32)>, pos : (i32,i32)) -> u32 {
    if map[pos.0 as usize][pos.1 as usize] == 9 && !already.contains(&pos) {
        already.insert(pos);
        return 1;
    }
    let mut local_score = 0;
    for d in DIR {
        let new = (pos.0 + d.0, pos.1 + d.1);
        if !is_outside(map, new) && map[pos.0 as usize][pos.1 as usize]+1 == map[new.0 as usize][new.1 as usize]  {
            local_score += count_rando_path(map, already, new);
        }
    }
    local_score
}
fn count_rando_path_p2(map : &Vec<Vec<u8>>, pos : (i32,i32)) -> u32 {
    if map[pos.0 as usize][pos.1 as usize] == 9 {
        return 1;
    }
    let mut local_score = 0;
    for d in DIR {
        let new = (pos.0 + d.0, pos.1 + d.1);
        if !is_outside(map, new) && map[pos.0 as usize][pos.1 as usize]+1 == map[new.0 as usize][new.1 as usize]  {
            local_score += count_rando_path_p2(map, new);
        }
    }
    local_score
}

fn is_outside(map : &Vec<Vec<u8>>, pos:(i32,i32)) -> bool {
    pos.0 < 0 || pos.1 < 0 || pos.0 >= map.len() as i32 || pos.1 >= map[0].len() as i32
}