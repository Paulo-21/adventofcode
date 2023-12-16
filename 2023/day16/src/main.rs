use std::{fs, collections::{HashMap, HashSet}, thread::current, cmp::max};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn p1() {
    //let file = fs::read_to_string("example").unwrap();
    let file = fs::read_to_string("input").unwrap();
    let mut map : Vec<Vec<char>> = Vec::new();
    let width = file.lines().next().unwrap().len() as i32;
    let mut height = 0;
    let mut already_pass = HashSet::<(i32, i32, Direction)>::new();
    for line in file.lines() {
        map.push(line.chars().collect());
        height += 1;
    }
    let mut energized = vec![vec![false;width as usize]; height as usize];
    let mut file : Vec<(i32, i32, Direction)> = Vec::from([(0, 0, Direction::Right)]);
    let mut i = 0;
    while file.len() > 0 {
        let current = file.pop().unwrap();
        match already_pass.get(&current) {
            Some(_x) => {
                continue;
            }, None => {
                already_pass.insert(current.clone());
            }
        }
        if current.0 >= height || current.0 < 0 || current.1 >= width || current.1 < 0 {
            continue;
        }
        let current_direction = current.2;
        energized[current.0 as usize][current.1 as usize] = true;
        //print_map(&energized);
        //println!("{}{}", current.0, current.1);
        match map[current.0 as usize][current.1 as usize] {
            '.' => {
                match current_direction {
                    Direction::Up => {
                        file.push((current.0-1, current.1, current_direction));
                    },
                    Direction::Down => {
                        file.push((current.0+1, current.1, current_direction));
                    },
                    Direction::Left => {
                        file.push((current.0, current.1-1, current_direction));
                    },
                    Direction::Right => {
                        file.push((current.0, current.1+1, current_direction));
                    },
                }
            },
            '-' => {
                match current_direction {
                    Direction::Up | Direction::Down => {
                        file.push((current.0, current.1-1, Direction::Left));
                        file.push((current.0, current.1+1, Direction::Right));
                    },
                    Direction::Left => {
                        file.push((current.0, current.1-1, current_direction));
                    },
                    Direction::Right => {
                        file.push((current.0, current.1+1, current_direction));
                    }
                }
                
            },
            '|' => {
                match current_direction {
                    Direction::Left | Direction::Right => {
                        file.push((current.0-1, current.1, Direction::Up));
                        file.push((current.0+1, current.1, Direction::Down));
                    },
                    Direction::Up => {
                        file.push((current.0-1, current.1, current_direction));
                    },
                    Direction::Down => {
                        file.push((current.0+1, current.1, current_direction));
                    }
                }
            },
            '/' => {
                match current_direction {
                    Direction::Left => {
                        file.push((current.0+1, current.1, Direction::Down));
                    },
                    Direction::Up => {
                        file.push((current.0, current.1+1, Direction::Right));
                    },
                    Direction::Right => {
                        file.push((current.0-1, current.1, Direction::Up));
                    },
                    Direction::Down => {
                        file.push((current.0, current.1-1, Direction::Left));
                    }
                }
            },
            '\\' => {
                match current_direction {
                    Direction::Left => {
                        file.push((current.0-1, current.1, Direction::Up));
                    },
                    Direction::Up => {
                        file.push((current.0, current.1-1, Direction::Left));
                    },
                    Direction::Right => {
                        file.push((current.0+1, current.1, Direction::Down));
                    },
                    Direction::Down => {
                        file.push((current.0, current.1+1, Direction::Right));
                    }
                }
            }
            _=> {}
        }

        i+=1;
    }
    let mut res = 0;
    for v in energized {
        let _ = v.iter().for_each(|x| res+= (*x) as i32);
        for c in v {
            if c ==true {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    println!("{}", res);
}
fn p2() {
    //let file = fs::read_to_string("example").unwrap();
    let file = fs::read_to_string("input").unwrap();
    let mut map : Vec<Vec<char>> = Vec::new();
    let width = file.lines().next().unwrap().len() as i32;
    let mut height = 0;
    
    let mut possible_start = Vec::new();
    for line in file.lines() {      
        map.push(line.chars().collect());
        height += 1;

    }
    for y in 0..height {
        possible_start.push((y,0, Direction::Right));
        possible_start.push((y,width-1, Direction::Left));
    }
    for x in 0..width {
        possible_start.push((0,x, Direction::Down));
        possible_start.push((height-1,x, Direction::Up));
    }
    let mut max_res = 0;
    for start in possible_start {
        let res = process_beam_from(&map, start, height, width);
        //println!("{:?} -> {}", start , res);
        max_res = max(max_res, res);
    }
    
    println!("{}", max_res);
}

fn process_beam_from(map : &Vec<Vec<char>>, start :(i32, i32, Direction), height : i32, width : i32 ) -> i32 {
    let mut already_pass = HashSet::<(i32, i32, Direction)>::new();
    let mut energized = vec![vec![false;width as usize]; height as usize];
    let mut file : Vec<(i32, i32, Direction)> = Vec::from([start]);
    let mut _i = 0;
    while file.len() > 0 {
        let current = file.pop().unwrap();
        match already_pass.get(&current) {
            Some(_x) => {
                continue;
            }, None => {
                already_pass.insert(current.clone());
            }
        }
        if current.0 >= height || current.0 < 0 || current.1 >= width || current.1 < 0 {
            continue;
        }
        let current_direction = current.2;
        energized[current.0 as usize][current.1 as usize] = true;
        //print_map(&energized);
        //println!("{}{}", current.0, current.1);
        match map[current.0 as usize][current.1 as usize] {
            '.' => {
                match current_direction {
                    Direction::Up => {
                        file.push((current.0-1, current.1, current_direction));
                    },
                    Direction::Down => {
                        file.push((current.0+1, current.1, current_direction));
                    },
                    Direction::Left => {
                        file.push((current.0, current.1-1, current_direction));
                    },
                    Direction::Right => {
                        file.push((current.0, current.1+1, current_direction));
                    },
                }
            },
            '-' => {
                match current_direction {
                    Direction::Up | Direction::Down => {
                        file.push((current.0, current.1-1, Direction::Left));
                        file.push((current.0, current.1+1, Direction::Right));
                    },
                    Direction::Left => {
                        file.push((current.0, current.1-1, current_direction));
                    },
                    Direction::Right => {
                        file.push((current.0, current.1+1, current_direction));
                    }
                }
                
            },
            '|' => {
                match current_direction {
                    Direction::Left | Direction::Right => {
                        file.push((current.0-1, current.1, Direction::Up));
                        file.push((current.0+1, current.1, Direction::Down));
                    },
                    Direction::Up => {
                        file.push((current.0-1, current.1, current_direction));
                    },
                    Direction::Down => {
                        file.push((current.0+1, current.1, current_direction));
                    }
                }
            },
            '/' => {
                match current_direction {
                    Direction::Left => {
                        file.push((current.0+1, current.1, Direction::Down));
                    },
                    Direction::Up => {
                        file.push((current.0, current.1+1, Direction::Right));
                    },
                    Direction::Right => {
                        file.push((current.0-1, current.1, Direction::Up));
                    },
                    Direction::Down => {
                        file.push((current.0, current.1-1, Direction::Left));
                    }
                }
            },
            '\\' => {
                match current_direction {
                    Direction::Left => {
                        file.push((current.0-1, current.1, Direction::Up));
                    },
                    Direction::Up => {
                        file.push((current.0, current.1-1, Direction::Left));
                    },
                    Direction::Right => {
                        file.push((current.0+1, current.1, Direction::Down));
                    },
                    Direction::Down => {
                        file.push((current.0, current.1+1, Direction::Right));
                    }
                }
            }
            _=> {}
        }
        _i+=1;
    }
    let mut res = 0;
    for v in energized {
        let _ = v.iter().for_each(|x| res+= (*x) as i32);
    }
    res
}

fn print_map (map : &Vec<Vec<bool>>) {
    for v in map {
        for c in v {
            if *c ==true {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    //p1();
    p2();
}
