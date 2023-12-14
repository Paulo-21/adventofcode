use std::{fs, usize, collections::HashMap};

enum Cardipoint {
    North,
    South,
    West,
    East,
}

fn p1() {
    let mut res = 0;
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut round_rocks = Vec::new();
    let mut map : Vec<Vec<char>> = Vec::new();
    for (y, line) in file.lines().enumerate() {
        for (x, unit) in line.chars().enumerate() {
            if unit == 'O' {
                round_rocks.push((y, x));
            }
        }
        map.push(line.chars().collect());
    }
    for rock in &mut round_rocks {
        if rock.0 == 0 {
            continue;
        }
        let mut new_y = rock.0.checked_sub(1).unwrap();
        let old_y = rock.0;

        loop  {
            if map[new_y][rock.1] != '.' {
                rock.0 = new_y+1;
                break;
            }
            match new_y.checked_sub(1) {
                Some(x) => new_y = x,
                None=> {
                    rock.0 = 0;
                    break;
                }
            }
        }
        map[old_y][rock.1] = '.'; 
        map[rock.0][rock.1] = 'O';
    }
    let leny = map.len();
    for rock in round_rocks {
        res += leny - rock.0;
    }
    for v in map {
        for c in v {
            print!("{c}");
        }
        //println!();
    }
    //println!("{}", res);
}


fn p2() {
    let mut hashmap : HashMap<String, (usize,usize, usize)> = HashMap::new();
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut round_rocks = Vec::new();
    let mut map : Vec<Vec<char>> = Vec::new();
    for (y, line) in file.lines().enumerate() {
        for (x, unit) in line.chars().enumerate() {
            if unit == 'O' {
                round_rocks.push((y, x));
            }
        }
        map.push(line.chars().collect());
    }
    let mut nb_step = 1;
    let mut jusqua = 1000000000;
    let mut loop_value = Vec::new();
    for _i in 0..jusqua {
        cycle(&mut round_rocks, &mut map);
        let key = compute_key_from_map(&map);
        
        match hashmap.get_mut(&key) {
            Some(x) => {
                if x.1 == 1 {
                    println!("FOUND");
                    println!("{}", nb_step);
                    println!("{:?}", x);
                    loop_value.push(x.clone());
                    //println!("{}", (nb_step-x.2)%(jusqua-x.2));
                }
                x.1+=1;
                if x.1 == 3 {
                    //println!("{:?}", loop_value);
                    println!("{:?}", loop_value[(jusqua-loop_value[0].2)%(loop_value.len())].0);
                    break;
                }
            },
            None => {
                let load = compute_load(&round_rocks, map.len());
                hashmap.insert(key, (load, 1, nb_step));
            }
        }
        nb_step +=1;
        //print_map(&map);
        //println!(); 
    }
    //let res = compute_load(&round_rocks, map.len());
    //1000000000
    //println!("{}", res);
}
fn compute_key_from_map (map : &Vec<Vec<char>>) -> String {
    let mut key = String::with_capacity(map[0].len() * map.len());
    for v in map {
        key.push_str(v.iter().collect::<String>().as_str());
    }
    key
}
fn tilte_to_north(round_rocks : &mut Vec<(usize,usize)>, map: &mut Vec<Vec<char>> ) {
    for rock in round_rocks {
        if rock.0 == 0 { continue; }
        let mut new_y = rock.0.checked_sub(1).unwrap();
        let old_y = rock.0;
        loop  {
            if map[new_y][rock.1] != '.' {
                rock.0 = new_y+1;
                break;
            }
            match new_y.checked_sub(1) {
                Some(x) => new_y = x,
                None=> {
                    rock.0 = 0;
                    break;
                }
            }
        }
        map[old_y][rock.1] = '.'; 
        map[rock.0][rock.1] = 'O';
    }
}
fn tilte_to_south(round_rocks : &mut Vec<(usize,usize)>, map: &mut Vec<Vec<char>> ) {
    for rock in round_rocks {
        if rock.0 == map.len()-1 { continue; }
        let mut new_y = rock.0 + 1;
        let old_y = rock.0;
        loop  {
            if map[new_y][rock.1] != '.' {
                rock.0 = new_y-1;
                break;
            }
            new_y += 1;
            if new_y >= map.len() { rock.0 = new_y-1; break;}
        }
        map[old_y][rock.1] = '.'; 
        map[rock.0][rock.1] = 'O';
    }
}
fn tilte_to_west(round_rocks : &mut Vec<(usize,usize)>, map: &mut Vec<Vec<char>> ) {
    for rock in round_rocks {
        if rock.1 == 0 { continue; }
        let mut new_x = rock.1.checked_sub(1).unwrap();
        let old_x = rock.1;
        loop  {
            if map[rock.0][new_x] != '.' {
                rock.1 = new_x+1;
                break;
            }
            match new_x.checked_sub(1) {
                Some(x) => new_x = x,
                None=> {
                    rock.1 = 0;
                    break;
                }
            }
        }
        map[rock.0][old_x] = '.'; 
        map[rock.0][rock.1]= 'O';
    }
}
fn tilte_to_east(round_rocks : &mut Vec<(usize,usize)>, map: &mut Vec<Vec<char>> ) {
    for rock in round_rocks {
        if rock.1 == map[0].len()-1 { continue; }
        let mut new_x = rock.1 +1;
        let old_x = rock.1;
        loop  {
            if map[rock.0][new_x] != '.' {
                rock.1 = new_x-1;
                break;
            }
            new_x +=1;
            if new_x >= map[0].len() {
                rock.1 =  new_x -1;
                break;
            }
        }
        map[rock.0][old_x] = '.'; 
        map[rock.0][rock.1] = 'O';
    }
}
fn cycle (round_rocks : &mut Vec<(usize,usize)>, map: &mut Vec<Vec<char>>) {
    get_coord_up_down(&map, round_rocks);
    tilte_to_north(round_rocks, map);
    //println!("NORTH");
    //print_map(map);
    get_coord_left_right(&map, round_rocks);
    tilte_to_west(round_rocks, map);
    //println!("WEST");
    //print_map(map);
    get_coord_up_down(&map, round_rocks);
    round_rocks.reverse();
    tilte_to_south(round_rocks, map);
    //println!("SOUTH");
    //print_map(map);
    get_coord_left_right(&map, round_rocks);
    round_rocks.reverse();
    tilte_to_east(round_rocks, map);
    //println!("East");
    //print_map(map);
}
fn get_coord_up_down(map : &Vec<Vec<char>>, round_rocks : &mut Vec<(usize,usize)>) {
    round_rocks.clear();
    for (i,v) in map.iter().enumerate() {
        for (k, c) in v.iter().enumerate() {
            if *c == 'O' {
                round_rocks.push((i,k));
            }
        }
    }
}
fn get_coord_left_right(map : &Vec<Vec<char>>, round_rocks : &mut Vec<(usize,usize)>) {
    round_rocks.clear();
    let mut i = 0;
    while i < map[0].len() {
        let mut k = 0;
        while k < map.len() {
            if map[k][i] == 'O' {
                round_rocks.push((k,i));
            } 
            k+=1;
        }
        i+=1;
    }
}
fn compute_load(round_rocks : &Vec<(usize,usize)>, ns_dist : usize) -> usize {
    let mut res = 0;
    for rock in round_rocks {
        res += ns_dist - rock.0;
    }
    res
}

fn print_map(map : &Vec<Vec<char>>) {
    for v in map {
        for c in v {
            print!("{c}");
        }
        //println!();
    }
    //println!();
}
fn main() {
    //p1();
    p2();
}
