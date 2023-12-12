use std::fs;
use colored::Colorize;

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    UNDEC,
    IN,
    OUT,
    PATH
}

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    println!("{}", file);
    println!();
    let mut map : Vec<Vec<char>> = Vec::new();
    let mut pos_s = (0, 0);
    for (index, line) in file.lines().enumerate() {
        map.push(line.chars().collect());
        if line.contains('S') {
            pos_s.0 = index;
            pos_s.1 = line.chars().position(|c| c == 'S').unwrap();
        }
    }

    let mut to_visit : Vec<(usize, usize, Direction)> = 
    Vec::from([
        (pos_s.0-1,pos_s.1,Direction::UP),
        (pos_s.0+1,pos_s.1,Direction::DOWN),
        (pos_s.0,pos_s.1-1,Direction::LEFT),
        (pos_s.0,pos_s.1+1,Direction::RIGHT),
        ]);
    let mut already_visit = vec![false;140*140];
    //let mut already_visit = vec![0usize;140*140];
    let mut nb_step = 0;
    while to_visit.len() > 0 {
        let node = to_visit.pop().unwrap();
        //println!("{} {} {:?}", node.0, node.1, node.2);
        match get_next_nodes(&map, &node) {
            Some(n) => {
                to_visit.push(n);
            },
            None => {     }
        }
        already_visit[node.0*140 + node.1] = true;
        if map[node.0][node.1] == 'S' && nb_step > 0 {
            println!("{}", nb_step);
            println!("{}", nb_step/2);
            break;
        }
        nb_step += 1;
        //to_visit.remove(0);
    }

}
fn get_next_nodes(map : &Vec<Vec<char>>, from_node : &(usize, usize, Direction)) -> Option<(usize, usize, Direction)> {
    //println!("{}", map[from_node.0][from_node.1]);
    
    match map[from_node.0][from_node.1] {
        '|' => {
            match from_node.2 {
                Direction::DOWN => {
                    return Some((from_node.0+1, from_node.1, from_node.2 ));
                },
                Direction::UP => {
                    return Some((from_node.0-1, from_node.1, from_node.2 ));
                },
                _ => { return None; }
            }
        },
        '-' => {
            match from_node.2 {
                Direction::LEFT => {
                    return Some((from_node.0, from_node.1-1, from_node.2 ));
                },
                Direction::RIGHT => {
                    return Some((from_node.0, from_node.1+1, from_node.2 ));
                },
                _ => { return None; }
            }
        },
        'L' => {
            match from_node.2 {
                Direction::DOWN => {
                    return Some((from_node.0, from_node.1+1, Direction::RIGHT ));
                },
                Direction::LEFT => {
                    return Some((from_node.0-1, from_node.1, Direction::UP ));
                },
                _ => { return None; }
            }
        },
        'J' => {
            match from_node.2 {
                Direction::DOWN => {
                    return Some((from_node.0, from_node.1-1, Direction::LEFT ));
                },
                Direction::RIGHT => {
                    return Some((from_node.0-1, from_node.1, Direction::UP ));
                },
                _ => { return None; }
            }
        },
        '7' => {
            match from_node.2 {
                Direction::UP => {
                    return Some((from_node.0, from_node.1-1, Direction::LEFT ));
                },
                Direction::RIGHT => {
                    return Some((from_node.0+1, from_node.1, Direction::DOWN ));
                },
                _ => { return None; }
            }
        },
        'F' => {
            match from_node.2 {
                Direction::UP => {
                    return Some((from_node.0, from_node.1+1, Direction::RIGHT ));
                },
                Direction::LEFT => {
                    return Some((from_node.0+1, from_node.1, Direction::DOWN ));
                },
                _ => { return None; }
            }
        },
        'S'=>{
            println!("SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS");
            return None;
        }
        _=> {
            return None;
        }
    }

}

fn get_next_nodes2(map : &Vec<Vec<char>>, from_node : &(usize, usize, Direction), alvisit : &mut Vec<Vec<TileType>>) -> Option<(usize, usize, Direction)> {
    //println!("{}", map[from_node.0][from_node.1]);
    
    match map[from_node.0][from_node.1] {
        '|' => {
            match from_node.2 {
                Direction::DOWN => {
                    if alvisit[from_node.0][from_node.1-1] != TileType::PATH {
                        alvisit[from_node.0][from_node.1-1] = TileType::IN;
                    }
                    if alvisit[from_node.0][from_node.1+1] != TileType::PATH {
                        alvisit[from_node.0][from_node.1+1] = TileType::OUT;
                    }
                    return Some((from_node.0+1, from_node.1, from_node.2 ));
                },
                Direction::UP => {
                    if alvisit[from_node.0][from_node.1-1] != TileType::PATH {
                        alvisit[from_node.0][from_node.1-1] = TileType::OUT;
                    }
                    if alvisit[from_node.0][from_node.1+1] != TileType::PATH {
                        alvisit[from_node.0][from_node.1+1] = TileType::IN;
                    }
                    return Some((from_node.0-1, from_node.1, from_node.2 ));
                },
                _ => { return None; }
            }
        },
        '-' => {
            match from_node.2 {
                Direction::LEFT => {
                    if alvisit[from_node.0+1][from_node.1] != TileType::PATH {
                        alvisit[from_node.0+1][from_node.1] = TileType::OUT;
                    }
                    if alvisit[from_node.0-1][from_node.1] != TileType::PATH {
                        alvisit[from_node.0-1][from_node.1] = TileType::IN;
                    }
                    return Some((from_node.0, from_node.1-1, from_node.2 ));
                },
                Direction::RIGHT => {
                    if alvisit[from_node.0+1][from_node.1] != TileType::PATH {
                        alvisit[from_node.0+1][from_node.1] = TileType::IN;
                    }
                    if alvisit[from_node.0-1][from_node.1] != TileType::PATH {
                        alvisit[from_node.0-1][from_node.1] = TileType::OUT;
                    }
                    return Some((from_node.0, from_node.1+1, from_node.2 ));
                },
                _ => { return None; }
            }
        },
        'L' => {
            match from_node.2 {
                Direction::DOWN => {
                    if alvisit[from_node.0+1][from_node.1] != TileType::PATH {
                        alvisit[from_node.0+1][from_node.1] = TileType::IN;
                    }
                    if alvisit[from_node.0][from_node.1-1] != TileType::PATH {
                        alvisit[from_node.0][from_node.1-1] = TileType::IN;
                    }
                    /*if alvisit[from_node.0][from_node.1+1] != TileType::PATH {
                        alvisit[from_node.0][from_node.1+1] = TileType::OUT;
                    }*/
                    return Some((from_node.0, from_node.1+1, Direction::RIGHT ));
                },
                Direction::LEFT => {
                    if alvisit[from_node.0+1][from_node.1] != TileType::PATH {
                        alvisit[from_node.0+1][from_node.1] = TileType::OUT;
                    }
                    if alvisit[from_node.0][from_node.1-1] != TileType::PATH {
                        alvisit[from_node.0][from_node.1-1] = TileType::OUT;
                    }
                    /*if alvisit[from_node.0][from_node.1+1] != TileType::PATH {
                        alvisit[from_node.0][from_node.1+1] = TileType::IN;
                    }*/
                    return Some((from_node.0-1, from_node.1, Direction::UP ));
                },
                _ => { return None; }
            }
        },
        'J' => {
            match from_node.2 {
                Direction::DOWN => {
                    if alvisit[from_node.0+1][from_node.1] != TileType::PATH {
                        alvisit[from_node.0+1][from_node.1] = TileType::OUT;
                    }
                    if alvisit[from_node.0][from_node.1+1] != TileType::PATH {
                        alvisit[from_node.0][from_node.1+1] = TileType::OUT;
                    }
                    return Some((from_node.0, from_node.1-1, Direction::LEFT ));
                },
                Direction::RIGHT => {
                    if alvisit[from_node.0+1][from_node.1] != TileType::PATH {
                        alvisit[from_node.0+1][from_node.1] = TileType::IN;
                    }
                    if alvisit[from_node.0][from_node.1+1] != TileType::PATH {
                        alvisit[from_node.0][from_node.1+1] = TileType::IN;
                    }
                    return Some((from_node.0-1, from_node.1, Direction::UP ));
                },
                _ => { return None; }
            }
        },
        '7' => {
            match from_node.2 {
                Direction::UP => {
                    return Some((from_node.0, from_node.1-1, Direction::LEFT ));
                },
                Direction::RIGHT => {
                    return Some((from_node.0+1, from_node.1, Direction::DOWN ));
                },
                _ => { return None; }
            }
        },
        'F' => {
            match from_node.2 {
                Direction::UP => {
                    return Some((from_node.0, from_node.1+1, Direction::RIGHT ));
                },
                Direction::LEFT => {
                    return Some((from_node.0+1, from_node.1, Direction::DOWN ));
                },
                _ => { return None; }
            }
        },
        'S'=>{
            println!("SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS");
            return None;
        }
        _=> {
            return None;
        }
    }

}
fn p2() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example2").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut map : Vec<Vec<char>> = Vec::new();
    let mut pos_s = (0, 0);
    for (index, line) in file.lines().enumerate() {
        map.push(line.chars().collect());
        if line.contains('S') {
            pos_s.0 = index;
            pos_s.1 = line.chars().position(|c| c == 'S').unwrap();
        }
    }

    let mut to_visit : Vec<(usize, usize, Direction)> = 
    Vec::from([
        (pos_s.0-1,pos_s.1,Direction::UP),
        (pos_s.0+1,pos_s.1,Direction::DOWN),
        (pos_s.0,pos_s.1-1,Direction::LEFT),
        (pos_s.0,pos_s.1+1,Direction::RIGHT),
        ]);
    let mut already_visit = vec![vec![TileType::UNDEC;140]; 140];
    //let mut already_visit = vec![0usize;140*140];
    let mut nb_step = 0;
    while to_visit.len() > 0 {
        let node = to_visit.pop().unwrap();
        //println!("{} {} {:?}", node.0, node.1, node.2);
        match get_next_nodes(&map, &node) {
            Some(n) => {
                to_visit.push(n);
            },
            None => {     }
        }
        already_visit[node.0][node.1] = TileType::PATH;
        if map[node.0][node.1] == 'S' && nb_step > 0 {
            println!("{}", nb_step);
            println!("{}", nb_step/2);
            break;
        }
        nb_step += 1;
        //to_visit.remove(0);
    }
    print_map(&map, &already_visit);
    let mut to_visit : Vec<(usize, usize, Direction)> = 
    Vec::from([
        (pos_s.0-1,pos_s.1,Direction::UP),
        (pos_s.0+1,pos_s.1,Direction::DOWN),
        (pos_s.0,pos_s.1-1,Direction::LEFT),
        (pos_s.0,pos_s.1+1,Direction::RIGHT),
        ]);
    while to_visit.len() > 0 {
        let node = to_visit.pop().unwrap();
        //println!("{} {} {:?}", node.0, node.1, node.2);
        match get_next_nodes2(&map, &node, &mut already_visit) {
            Some(n) => {
                to_visit.push(n);
            },
            None => {     }
        }
        //already_visit[node.0][node.1] = TileType::PATH;
        if map[node.0][node.1] == 'S' && nb_step > 0 {
            println!("{}", nb_step);
            println!("{}", nb_step/2);
            break;
        }
        nb_step += 1;
        //to_visit.remove(0);
    }
    flood(&mut already_visit);
    print_map(&map, &already_visit);
    println!("NB IN : {}", count_in(&already_visit));
}

fn flood(already_visit : &mut Vec<Vec<TileType>>) {
    let mut y = 0; 
    let mut x = 0;
    while y < 140 {
        if already_visit[y][x] == TileType::OUT {
            if already_visit[y-1][x] != TileType::PATH {
                already_visit[y-1][x] = TileType::OUT;
            }
            if already_visit[y+1][x] != TileType::PATH {
                already_visit[y+1][x] = TileType::OUT;
            }
            /*if x > 0 && already_visit[y][x-1] != TileType::PATH {
                already_visit[y][x-1] = TileType::OUT;
            }
            if x < 139 && already_visit[y][x+1] != TileType::PATH {
                already_visit[y][x+1] = TileType::OUT;
            }*/
        }
        x+=1;
        if x >= 140 {
            y +=1;
            x = 0;
        }

    }
     
}

fn print_map(map : &Vec<Vec<char>>, already_visit : &Vec<Vec<TileType>>, ) {
    for (i, line) in map.iter().enumerate() {
        for (k, c) in line.iter().enumerate() {
            match already_visit[i][k] {
                TileType::UNDEC => {
                    print!("{}", c.to_string());
                }
                TileType::IN => {
                    print!("{}", c.to_string().green());
                }
                TileType::OUT => {
                    print!("{}", c.to_string().red());
                }
                TileType::PATH => {
                    print!("{}", c.to_string().blue());
                }
            }
        }
        println!();
    }
}
fn count_in(already_visit : &Vec<Vec<TileType>> ) -> i32 {
    let mut res = 0;
    for line in already_visit {
        for c in line {
            match c {
                TileType::OUT => {
                    res += 1;
                },
                _=>{  }
            }
        }
    }
    res
}


fn main() {
    //p1();
    p2();
}
