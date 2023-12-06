use std::fs;

fn p1 () {
    let file = fs::read_to_string("input").unwrap();
    let mut iter = file.lines();
    let seed_ligne = iter.next().unwrap();
    let mut value :Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut index = 0;
    for line in iter {
        if line == "" {
            continue;
        }
        if !line.chars().next().unwrap().is_digit(10) {
            if value.len() > 0 {
                index+=1;
            }
            value.push(Vec::new());
        }
        else {
            let mut r = line.split_ascii_whitespace();
            let d : u64 = r.next().unwrap().parse().unwrap();
            let s : u64 = r.next().unwrap().parse().unwrap();
            let r :u64 = r.next().unwrap().parse().unwrap();
            let mut index_to_insert = 0;
            for el in &value[index] {
                if s > el.1 {
                    break;
                }
                index_to_insert+=1;
            }
            value[index].insert(index_to_insert, (d,s,r));
        }
    }
    let mut minimal = u64::MAX;
    let a = seed_ligne.split(':').nth(1).unwrap().split_ascii_whitespace();
    for seed_number in a {
        let mut point = seed_number.parse().unwrap();
        for v in &value {
            print!("{}", point);
            point = convert_value(v, point);
            println!(" -> {}", point);
        }
        if point < minimal {
            minimal = point;
            //println!("{minimal}");
        }
        //break;
    }
    println!("{}", minimal);
}
fn p2 () {
    let file = fs::read_to_string("input").unwrap();
    let mut iter = file.lines();
    let seed_ligne = iter.next().unwrap();
    let mut value :Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut index = 0;
    for line in iter {
        if line == "" {
            continue;
        }
        if !line.chars().next().unwrap().is_digit(10) {
            if value.len() > 0 {
                index+=1;
            }
            value.push(Vec::new());
        }
        else {
            let mut r = line.split_ascii_whitespace();
            let d : u64 = r.next().unwrap().parse().unwrap();
            let s : u64 = r.next().unwrap().parse().unwrap();
            let r :u64 = r.next().unwrap().parse().unwrap();
            let mut index_to_insert = 0;
            for el in &value[index] {
                if s > el.1 {
                    break;
                }
                index_to_insert+=1;
            }
            value[index].insert(index_to_insert, (d,s,r));
        }
    }
    let mut minimal = u64::MAX;
    let seed_str : Vec<&str> = seed_ligne.split(':').nth(1).unwrap().split_ascii_whitespace().collect();
    let mut seed_numbers : Vec<(u64,u64)> = Vec::new();
    let mut i = 0;

    while i < seed_str.len() {
        let a = seed_str[i].parse::<u64>().unwrap();
        let b = seed_str[i+1].parse::<u64>().unwrap();
        seed_numbers.push((a,b));
        i+=2;
    }
    for v in &value {
        let mut new_numbers = Vec::new();
        while !seed_numbers.is_empty() {
            let point = seed_numbers.pop().unwrap();
            //print!("{:?}", point);
            let (new_point, new_one) = convert_value_p2(v, point);
            if let Some(x) = new_one {
                seed_numbers.push(x);
            }
            new_numbers.push(new_point);
            //println!(" -> {:?}", point);
        }
        seed_numbers = new_numbers;
        //break;
    }

    for a in seed_numbers {
        if a.0 < minimal {
            minimal = a.0;
        }
    }
    println!("{}", minimal);
}

fn main() {
    //p1();
    p2();
}

fn convert_value_p2( map : &Vec<(u64,u64,u64)>, source_v : (u64,u64)) -> ((u64,u64), Option<(u64,u64)>) {
    
    
    for el in map {
        if el.1 <= source_v.0 && source_v.0 < el.1 + el.2 {
            let diff = source_v.0 - el.1;
            if el.1+el.2 < source_v.0 + source_v.1 {
                return ((el.0 + diff,  el.2-diff),Some((el.1+el.2,(source_v.1 + source_v.0) -(el.1+el.2) )));
            }
            return ((el.0 + diff,source_v.1),None);
        }
    }

    return (source_v, None);
}
fn convert_value( map : &Vec<(u64,u64,u64)>, source_v : u64) -> u64 {
    let mut i = 0;
    let mut find = false;
    let mut diff = 0;
    for el in map {
        if el.1 <= source_v && source_v < el.1 + el.2 {
            find = true;
            diff = source_v - el.1;
            break;
        }
        i+=1;
    }

    if find {
        return map[i].0 + diff;
    }
    return source_v;
}
