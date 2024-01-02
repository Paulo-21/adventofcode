use std::{fs, time::Instant, process::exit, collections::VecDeque};
#[derive(Debug, Clone)]
struct Block {
    start : [i32;3],
    end : [i32;3],
    label : char,
    support : Vec<usize>,
    support_by : Vec<usize>,
    id : usize,
}
fn p1(file : String) {
    //let file = fs::read_to_string("example").unwrap();
    let file = fs::read_to_string("input").unwrap();
    
    let mut all_block : Vec<Block> = Vec::with_capacity(3000);
    let mut label : u8 = 65;
    let mut id = 0;
    let mut max_h = 0;
    let mut res = 0;
    for brick in file.lines() {
        let mut s = brick.split('~');
        let start : Vec<i32> = s.next().unwrap().split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        let seconde : Vec<i32> = s.next().unwrap().split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        if seconde[2] > max_h {
            max_h = seconde[2];
        }
        let at = all_block.iter().position(|b| b.start[2] > start[2]);
        match at {
            Some(x) => {
                all_block.insert(x, Block { start : start.try_into().unwrap(), end :seconde.try_into().unwrap(), label : label as char , support : Vec::new(), support_by : Vec::new(), id});
            },
            None => {
                all_block.push(Block { start : start.try_into().unwrap(), end :seconde.try_into().unwrap(), label : label as char, support : Vec::new(), support_by : Vec::new(), id});
            }
        }
        if let Some(x) = label.checked_add(1) {
            label = x;
        }
        id+=1;
    }
    //exit(0);
    
    make_them_fall2(&mut all_block, max_h);
    
    //println!("{}", start.elapsed().as_micros());
    
}

fn make_them_fall2(all_block : &mut Vec<Block>, max_h : i32) {
    
    let mut fallen : Vec<Vec<Block>> = vec![Vec::new(); max_h as usize];
    for b in all_block {
        let h_s = b.start[2] as usize;
        //println!("{:?}", b);
        let mut k = (h_s-1) as i32;
        let diff = b.end[2] - b.start[2];
        while k >= 0 {
            //println!("{k}");
            let mut hited = false;
            for inf in &mut fallen[k as usize] {
                if hit(inf, b) && inf.end[2] <= b.start[2] {
                    inf.support.push(b.id);
                    b.support_by.push(inf.id);
                    hited = true;
                }
            }
            if hited {
                let mut copy = b.clone();
                copy.start[2] = k+1 ;
                copy.end[2] = k+diff+1;
                fallen[(k+diff) as usize +1].push(copy);
                break;
            }
            if k <= 1 && hited == false {
                let mut copy = b.clone();
                copy.start[2] = 1;
                copy.end[2] = 1+diff;
                fallen[1+diff as usize].push(copy);
                break;
            }
            k-=1;
        }
    }
    
    //print_fallen(&fallen);
    println!("{}", count_desintegrate2(&fallen));
    println!("{}", count_desintegrate2_p2(&fallen));
}

fn count_desintegrate2(fallen : &Vec<Vec<Block>>) -> i32 {
    let mut count = 0;
    for f in fallen {
        if f.len() <= 1 {
            if f.len() == 1 && f[0].support.is_empty() {
                count+=1;
            }
            continue;
        }
        for (k, b) in f.iter().enumerate() {
            let mut all_supported = true;
            
            for s in &b.support {
                let mut supported = false;
                for (i, other) in f.iter().enumerate() {
                    if i == k { continue; }
                    if other.support.contains(s) {
                        supported = true;
                        break;
                    }
                }
                if !supported {
                    all_supported = false;
                }
            }
            if all_supported {
                //println!("YES : {}", b.label);
                count+=1;
            }
        }
    }
    count
}
fn count_desintegrate2_p2(fallen : &Vec<Vec<Block>>) -> i32 {
    let mut count = 0;
    let mut all : Vec<Vec<usize>> = vec![Vec::new();100000];
    let mut all_by : Vec<Vec<usize>> = vec![Vec::new();100000];
    for f in fallen {
        for b in f {
            all[b.id] = b.support.clone();
            all_by[b.id] = b.support_by.clone();
        }
    }
    for f in fallen {
        for (k, b) in f.iter().enumerate() {
                //println!("NO : {}", b.label);
                let mut to_fall = Vec::with_capacity(100);
                //println!("label {}", b.id);
                let mut stack = VecDeque::from([b.id]);
                while !stack.is_empty() {
                    let next: usize = stack.pop_front().unwrap();
                    //println!("{}", next);
                    //println!("{}", next);
                    //println!(" fall   {:?}", to_fall);
                    //println!("supp by {:?}", all_by[next]);
                    if !to_fall.contains(&next) {
                        //println!("PUSH {}", next);
                        to_fall.push(next);
                        count+=1;
                        for toadd in all[next].iter() {
                            let mut contain = true;
                            for e in all_by[*toadd].iter() {
                                if !to_fall.contains(e) {
                                    contain = false;
                                    break;
                                }
                            }
                            if !to_fall.contains(toadd) && contain {
                                stack.push_back(*toadd);
                            }
                            
                        }
                    }
                }
                count-=1;
            
        }
    }
    count
}

fn hit(one : &Block, two : &Block) -> bool {
    let a = one.start[0] <= two.start[0] && two.start[0] <= one.end[0];
    let d = two.start[1] <= one.start[1] && one.start[1] <= two.end[1];
    let c = two.start[0] <= one.start[0] && one.start[0] <= two.end[0];
    let b = one.start[1] <= two.start[1] && two.start[1] <= one.end[1];
    (c && b) | (a && d)
}

fn print_fallen(all_block : &Vec<Vec<Block>>) {
    for v in all_block.iter().rev() {
        if v.is_empty() { continue; }
        for b in v {
            print!("{:?} || ", b);
        }
        println!();
        println!();
    }
}

fn main() {
    //let t = p1(fs::read_to_string("example").unwrap());
    p1(fs::read_to_string("input").unwrap());
}