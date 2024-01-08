use std::fs;

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    let mut s = file.split('-');
    let mut start : Vec<u32> = s.next().unwrap().chars().map(|c| c as u32 -48).collect();
    let end : Vec<u32> = s.next().unwrap().chars().map(|c| c as u32 -48).collect();
    start = vec![2,7,8,8,8,8];
    let res = rec(start, &end);
    println!("{}", res.0);
    println!("{}", res.1);
}
fn rec(mut start : Vec<u32>, end: &Vec<u32>) -> (u32, u32) {
    let mut res = 0;
    let mut res2 = 0;
    let l = start.len()-1;
    while !greater_than(&start, end) {
        let mut temp_l = l;
        loop {
            if start[temp_l] >= 9 {
                temp_l-=1;
            }
            else { break; }
        }
        start[temp_l] +=1;
        temp_l+=1;
        loop {
            if temp_l > l {
                break;
            }
            start[temp_l] = start[temp_l-1] ;
            temp_l+=1;
        }
        if valid2(&start) {
            println!("{:?}", start);
            res2 += 1;
        }
        if valid(&start) {
            //println!("{:?}", start);
            res += 1;
        }
    }
    (res, res2)
}
fn valid2(start : &Vec<u32>) -> bool {
    for d in start.iter() {
        let mut nocc = 0;
        start.iter().for_each(|c| if *c == *d { nocc +=1; });
        if nocc == 2 {
            return true;
        }
    }
    false
}
fn valid (start : &Vec<u32>) -> bool {
    let mut curr = start[0];
    for d in start[1..].iter() {
        if curr == *d {
            return true;
        }
        curr = *d;
    }
    false
}
fn greater_than(start : &Vec<u32>, end: &Vec<u32>)  -> bool {
    for (k, s) in start.iter().enumerate() {
        if *s == end[k] {
            continue;
        }
        else if *s < end[k] {
            return false;
        }
        else if *s > end[k] {
            return true;
        }
    }
    false
}

fn main() {
    p1();
}
