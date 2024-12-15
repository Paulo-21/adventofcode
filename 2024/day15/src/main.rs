use std::{collections::BTreeMap, fs};

fn main() {
    let dirm: BTreeMap<char, (i32,i32)> = BTreeMap::from([ ('v', (1,0)), ('^', (-1,0)), ('<', (0,-1)), ('>', (0,1)), ]); 
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    //let input = fs::read_to_string("small_example").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut instucts: Vec<char> = Vec::new(); 
    let mut instruct_mode = false;
    let mut robot_pos = (0,0);
    for (i,line) in input.lines().enumerate() {
        if line.is_empty() { instruct_mode = true; }
        if !instruct_mode {
            if let Some(rpos) = line.chars().position(|c| c == '@') {
                robot_pos = (i,rpos);
            }
            map.push(line.chars().collect());
        }
        else {
            instucts.extend(line.chars());
        }
    }
    //print_map(&map);
    //println!("{:?}", instucts);
    for m in instucts {
        let d = dirm[&m];
        //println!("Move {}:", m);
        if move_here(&mut map, robot_pos, d) {
            map[robot_pos.0][robot_pos.1] = '.';
            robot_pos.0 = (robot_pos.0 as i32 + d.0) as usize;
            robot_pos.1 = (robot_pos.1 as i32 + d.1) as usize;
            map[robot_pos.0][robot_pos.1] = '@';
        }
        //print_map(&map);
    }
    let score1 = score(&map);
    println!("{}", score1);
}
fn score(map : &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    for (i,line) in map.iter().enumerate() {
        for (k, c) in line.iter().enumerate() {
            if *c == 'O' { score += (100*i) + k; }
        }
    }
    score
}
fn move_here(map : &mut Vec<Vec<char>>, mut robot_pos : (usize,usize), d:(i32,i32)) -> bool {
    let mut tomove = Vec::with_capacity(2);
    loop {
        robot_pos.0 = (robot_pos.0 as i32 + d.0) as usize;
        robot_pos.1 = (robot_pos.1 as i32 + d.1) as usize;
        let next = map[robot_pos.0][robot_pos.1];
        if tomove.len() == 0 {
            tomove.push(robot_pos);
        }
        if next == '.' {
            tomove.push(robot_pos);
            break;
        }
        else if next == '#'{
            return false;
        }
    }
    let (fi, fj) = *tomove.first().unwrap();
    map[fi][fj] = '.';
    let (fi, fj) = tomove.pop().unwrap();
    map[fi][fj] = 'O';
    true
}

fn print_map(map : &Vec<Vec<char>>) {
    for line in map {
        for c in line {
            print!("{}", *c);
        }
        println!();
    }
}
