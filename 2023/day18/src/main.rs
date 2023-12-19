use std::{fs, collections::HashSet};
const TRANSFORM : [(i32,i32);4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn apply_direction(mut source : (i32, i32), dir : char) -> (i32, i32) {
    let nb_dir = match dir {
        'U' => 0,
        'D' => 1,
        'L' => 2,
        'R' => 3,
        _ => 0
    };
    source.0 += TRANSFORM[nb_dir].0;
    source.1 += TRANSFORM[nb_dir].1;
    return source;
}
fn get_in_and_out(source : (i32, i32), dir : char) -> ((i32, i32), (i32, i32)) {
    let mut out = source;
    let mut inside = source;
    let nb_dir_out = match dir {
        'U' => 2,
        'D' => 3,
        'L' => 1,
        'R' => 0,
        _ => 0
    };
    let nb_dir_in = match dir {
        'U' => 3,
        'D' => 2,
        'L' => 0,
        'R' => 1,
        _ => 0
    };
    out.0 += TRANSFORM[nb_dir_out].0;
    out.1 += TRANSFORM[nb_dir_out].1;
    inside.0 += TRANSFORM[nb_dir_in].0;
    inside.1 += TRANSFORM[nb_dir_in].1;
    return (out, inside);
}
fn get_four_dir(source : (i32, i32)) -> [(i32, i32);4] {
    [
        (source.0+TRANSFORM[0].0, source.1+TRANSFORM[0].1),
        (source.0+TRANSFORM[1].0, source.1+TRANSFORM[1].1),
        (source.0+TRANSFORM[2].0, source.1+TRANSFORM[2].1),
        (source.0+TRANSFORM[3].0, source.1+TRANSFORM[3].1),
    ]
}
fn flood (limit : &HashSet<(i32, i32)>, inside_cube : &mut HashSet<(i32, i32)>, outside_cube : &mut HashSet<(i32, i32)>) -> usize {
    let in2 = inside_cube.clone();
    let ou2 = outside_cube.clone();
    let mut flood_inside = Vec::from_iter(in2.to_owned());
    let mut flood_outside = Vec::from_iter(ou2.to_owned());
    let mut new_flood_inside = Vec::<(i32, i32)>::new();
    let mut new_flood_outside = Vec::<(i32, i32)>::new();
    let mut nb_in = flood_inside.len();
    let mut nb_out = flood_outside.len();
    while flood_inside.len() > 0 && flood_outside.len() > 0 {
        for new_in in &flood_inside {
            for possi in get_four_dir(*new_in) {
                if !limit.contains(&possi) && !inside_cube.contains(&possi) {
                    new_flood_inside.push(possi);
                    inside_cube.insert(possi);
                }
            }
        }
        for new_out in &flood_outside {
            for possi in get_four_dir(*new_out) {
                if !limit.contains(&possi) && !outside_cube.contains(&possi) {
                    new_flood_outside.push(possi);
                    outside_cube.insert(possi);
                }
            }
        }

        nb_in += new_flood_inside.len();
        nb_out += new_flood_outside.len();
        std::mem::swap(&mut flood_inside, &mut new_flood_inside);
        std::mem::swap(&mut flood_outside, &mut new_flood_outside);
        new_flood_inside.clear();
        new_flood_outside.clear();
    }

    if flood_inside.len() <= 0 {
        return nb_in;
    }
    else if flood_outside.len() <= 0 {
        return nb_out;
    }
    0
}
fn convert_to_dec(hexa : &str) -> i64 {
    let mut dec = 0;
    for (i, d) in hexa.char_indices() {
        let c  = d as u8;
        if c >= b'0' && c <= b'9' {
            dec += (d as i64 -48) * 16i64.pow((hexa.len()-1-i) as u32);
        }
        else if c >= b'a' && c <= b'f' {
            dec += ((d as i64 -97) +10) * 16i64.pow((hexa.len()-1-i) as u32);
        }
    }

    dec
}

fn p2() {
    let mut res = 0;
    //let file = fs::read_to_string("input").unwrap();
    let file = fs::read_to_string("example").unwrap();
    //let start = Instant::now();
    let mut nb_r = 0;
    let mut nb_l = 0;
    let mut nb_u = 0;
    let mut nb_d = 0;
    for line in file.lines() {
        let parse : Vec<&str> = line.split_ascii_whitespace().collect();
        let exa = &parse[2][2..7];
        let dir_nb = parse[2].chars().nth(7).unwrap() as i32 -48;
        
        match dir_nb {
            0 => nb_r += convert_to_dec(exa),
            1 => nb_d += convert_to_dec(exa),
            2 => nb_l += convert_to_dec(exa),
            3 => nb_u += convert_to_dec(exa),
            _ => {},
        };
    }
    println!("{}" , nb_r+nb_d);
    println!("{}" , nb_l+nb_u);
    println!("{}" , (nb_l+nb_u) * (nb_r+nb_d));
    
    println!("{}",   (res as f64 /2.0).powf(2.0) as i64 -res);
}
fn p1() {
    let file = fs::read_to_string("input").unwrap();
    
    let mut cube_out : HashSet<(i32, i32)> = HashSet::new();
    let mut cube_in : HashSet<(i32, i32)> = HashSet::new();
    let mut limit : HashSet<(i32, i32)> = HashSet::new();
    let mut current = (0, 0);
    for line in file.lines() {
        let parse : Vec<&str> = line.split_ascii_whitespace().collect();
        let dir = parse[0].chars().nth(0).unwrap();
        let nb = parse[1].parse::<i32>().unwrap();
        for _i in 0..nb {

            limit.insert(current);
            current = apply_direction(current, dir);
            let (curr_out, curr_in) = get_in_and_out(current,dir);
            if cube_in.contains(&current) {
                cube_in.remove(&current);
            }
            if cube_out.contains(&current) {
                cube_in.remove(&current);
            }
            if !limit.contains(&curr_out) {
                cube_out.insert(curr_out);
            }
            if !limit.contains(&curr_in) {
                cube_in.insert(curr_in);
            }
        }
    }
    let add = flood(&limit, &mut cube_in, &mut cube_out);
    println!("{:?}", limit.len()+add);
}


fn main() {
    p1();
    //p2();
}
