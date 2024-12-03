use std::{fmt::Display, fs};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down
}
impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::Up  => { write!(f, "UP")   },
            Dir::Down=> { write!(f, "DOWN") },
        }
    }
}

fn is_safe(first : i32, second : i32, dir : Dir) -> bool {
    match dir {
        Dir::Down => {
            //println!("{}, {} = {}", second, first, second-first);
            if (second - first) == -1 || (second - first) == -2 || (second - first) == -3 {
                return true;
            }
        },
        Dir::Up => {
            if (second - first) == 1 || (second - first) == 2 || (second - first) == 3 {
                return true;
            }
        }
    }
    false
}
fn main() {
    //let input = fs::read_to_string("exemple").unwrap();
    let input = fs::read_to_string("input").unwrap();
    let mut nb_safe = 0;
    for line in input.lines() {
        let nombres : Vec<&str> = line.split_ascii_whitespace().collect();
        let mut s = nombres.iter(); 
        let first = s.next().unwrap().parse::<i32>().unwrap();
        let second = s.next().unwrap().parse::<i32>().unwrap();

            if (second - first) == 1 || (second - first) == 2 || (second -first) == 3 {
                let mut last = second;
                let mut safe = true;
                for couille in s {
                    let current = couille.parse::<i32>().unwrap();
                    if (current - last) != 1 && (current - last) != 2 && (current - last) != 3 {
                        safe = false;
                        break;
                    }
                    last = current;
                }
                if safe {
                    nb_safe +=1;
                }
            }
            else if (first - second) == 1 || (first - second) == 2 || (first -second) == 3 {
                let mut last = second;
                let mut safe = true;
                for couille in s {
                    let current = couille.parse::<i32>().unwrap();
                    if (last - current) != 1 && (last - current) != 2 && (last - current) != 3 {
                        safe = false;
                        break;
                    }
                    last = current;
                }
                if safe {
                    nb_safe +=1;
                }
            }
    }
    println!("{}",nb_safe);

    /////////////////////////////////////////////P2////////////////////////////
    let mut res = 0;
    for line in input.lines() {
        let nombres : Vec<&str> = line.split_ascii_whitespace().collect();
        if is_safe_vec(&nombres) {
            res += 1;
            continue;
        }
        for i in 0..nombres.len() {
            let mut temp = nombres.clone();
            temp.remove(i);
            if is_safe_vec(&temp) {
                res+=1;
                break;
            }
        }
    }
    println!("{}",res);
}

fn is_safe_vec(vec : &[&str]) -> bool {
    let mut last = vec.first().unwrap().parse::<i32>().unwrap();
    let second = vec.iter().nth(1).unwrap().parse::<i32>().unwrap();
    let dir = if last < second { Dir::Up} else if last > second { Dir::Down} else {return false;};
    for s in vec[1..].iter() {
        let current = s.parse().unwrap();
        if !is_safe(last, current, dir) {
            return false;
        }
        last = current;
    }
    true
}
