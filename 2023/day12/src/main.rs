use std::{fs, collections::HashMap, time::Instant, cmp::max};
use colored::Colorize;

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    //let file = fs::read_to_string("custom").unwrap();
    let mut res = 0;
    let mut z = file.lines();
    for line in file.lines() {// z.skip(3) {
        let mut hashmap = HashMap::new();
        let split :Vec<&str> = line.split_ascii_whitespace().collect();
        let mut line_report = String::from(split[0]);
        let mut nume = String::from(split[1]);
        for _i in 1..5 {
            line_report.push('?');
            line_report.push_str(split[0]);
        }
        for _i in 1..5 {
            nume.push(',');
            nume.push_str(split[1]);
        }
        //let line_report = split[0];
        let start = Instant::now();
        let numbers_broken :Vec<usize> = nume.split(',').collect::<Vec<&str>>().iter().map(|n| (*n).parse::<usize>().unwrap()).collect();
        println!("{:?}", numbers_broken);
        println!("{}", line_report.green());
        let e = rec(&mut hashmap, line_report.chars().collect(), &numbers_broken[..], 0, 0);
        println!("res : {}", e.to_string().red());
        println!("time : {}ms", start.elapsed().as_millis().to_string().purple());
        res += e;
        
        //break;

    }

    println!("{}", res);
}


fn rec(
hashmap : &mut HashMap<(String, usize), usize>, line : Vec<char>, 
blocks : &[usize], blocks_index: usize, mut start : usize 
) -> usize {
    let len = match blocks.get(blocks_index) {
        Some(x) => {
            *x
        },
        None => { 
            //println!("1");
            if line.iter().filter(|&n| *n == '#').count() != blocks.iter().sum() {
                return 0;
            }
            return  1;
        }
    };
    let mut res = 0;

    /*HashMap */
    let mut space = vec![b' '; max(start as i32-1, 0) as usize];
    let mut hastag = vec![b'#'; len];
    space.append(&mut hastag);
    let key = (String::from_utf8(space).unwrap(), blocks_index);
    if let Some(hash_value) = hashmap.get(&key) {
        return *hash_value;
    }
    
    while start+len <= line.len() {
        let mut line_test = line.clone();
        
        if is_accepted(&mut line_test, start, len, blocks, blocks_index) {
            //println!("bl index : {}", blocks_index);
            //println!("{}", (&line_test).into_iter().collect::<String>());
            res += rec(hashmap, line_test, blocks, blocks_index+1, start+len+1);
        }
        start += 1;
    }
    hashmap.insert(key, res);
    res
}

fn is_accepted(line : &mut Vec<char>, start : usize, len : usize, blocks : &[usize], blocks_index: usize) -> bool {
    if let Some(_x) = &line[start..start+len].iter().find(|c| **c=='.') {
        //println!("FALSE1 {}", line.iter().collect::<String>());
        return false;
    }
    for i in start..start+len {
        line[i] = '#';
    }
    if start+len < line.len() && line[start+len] == '#' {
        //println!("FALSE2 {}", line.iter().collect::<String>());
        return false;
    }
    if start+len < line.len() {
        line[start+len] = '.';
    }
    
    if line[..start+len].iter().filter(|&n| *n == '#').count() > blocks[..blocks_index+1].iter().sum() {
        //println!("FALSE3 {}", line.iter().collect::<String>());
        return false;
    }
    //Does he now contains more '#' than possible ?

    true
}
fn main() {
    p1();
}
