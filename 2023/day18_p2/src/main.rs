use std::{fs, collections::{HashSet, btree_map::Range}};

const TRANSFORM : [(i64,i64);4] = [(0, 1), (1, 0), (0, -1), (-1, 0) ];
fn apply_direction(mut source : (i64, i64), dir : usize, range : i64) -> (i64, i64) {
    
    source.0 += TRANSFORM[dir].0 * (range);
    source.1 += TRANSFORM[dir].1 * (range);
    return source;
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

fn euclide (one : (i64, i64), two : (i64, i64)) -> i64 {
    (((one.0 - two.0).pow(2) + (one.1 - two.1).pow(2)) as f64).sqrt() as i64
}
fn p2() {
    let mut res = 0;
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut sommet1 = (0i64,0i64);
    let mut sommet2 = (0i64,0i64);
    let mut sum_range = 0;
    //let start = Instant::now();
    for line in file.lines() {
        let parse : Vec<&str> = line.split_ascii_whitespace().collect();
        //let dir_nb = convert_dir(parse[0].chars().nth(0).unwrap());
        //let range = parse[1].parse::<i64>().unwrap();
        
        let hexa = &parse[2][2..7];
        let dir_nb = (parse[2].chars().nth(7).unwrap() as i64 -48) as usize;
        let range = convert_to_dec(hexa);
        
        
        sum_range+= range;
        sommet2 = apply_direction(sommet1, dir_nb, range);
        
        res += (sommet1.0-1 + sommet2.0+1) * (sommet1.1-1 - sommet2.1+1);
        println!("{:?} {:?} ",  sommet1, sommet2);
        println!("dst : {}", euclide(sommet1, sommet2));
        sommet1 = sommet2;
    }
    println!("{}", (res /2) + (sum_range/2) +1);
}

fn convert_dir(c : char) -> usize {
    match c {
        'R' => { //R
            0       
        },
        'D' => {//D
            1
        },
        'L' => {//L
           2     
        },
        'U' => {//U
            3
        },
        _ => {0},
    }
}

fn main() {
    p2();
}
