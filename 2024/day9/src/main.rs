use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    let mut occuped = Vec::new();
    let mut free: Vec<(u32, Vec<(u32,u32)>)> = Vec::new();
    for (i,c) in input.chars().enumerate() {
        if i%2 == 0 { occuped.push(((i/2) as u32, c.to_digit(10).unwrap(), c.to_digit(10).unwrap())); }
        else { free.push((c.to_digit(10).unwrap(), Vec::new())); }
    }
    for (iter ,(id, nb, _)) in occuped.iter_mut().enumerate().rev() {
        for (iter_free,(still, to_fill)) in free.iter_mut().enumerate() {
            if *still > 0 && iter > iter_free {
                if *still < *nb {
                    to_fill.push((*still, *id));
                    *nb -= *still;
                    *still = 0;
                } else {
                    to_fill.push((*nb, *id));
                    *still -= *nb;
                    *nb = 0;
                    break;
                }
            }
        }
    }
    //debug(occuped.clone(), free.clone());
    let res1 = checksum(occuped.clone(), free.clone());
    println!();
    println!("6301895872542");
    p2();
}
fn p2() {
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    let mut occuped = Vec::new();
    let mut free: Vec<(u32, Vec<(u32,u32)>)> = Vec::new();
    for (i,c) in input.chars().enumerate() {
        if i%2 == 0 { occuped.push(((i/2) as u32, c.to_digit(10).unwrap(),c.to_digit(10).unwrap())); }
        else { free.push((c.to_digit(10).unwrap(), Vec::new())); }
    }
    for (iter ,(id, nb, nbsave)) in occuped.iter_mut().enumerate().rev() {
        for (iter_free,(still, to_fill)) in free.iter_mut().enumerate() {
            if *still > 0 && iter > iter_free {
                if *still >= *nb {
                    to_fill.push((*nb, *id));
                    *still -= *nb;
                    *nb = 0;
                    break;
                }
            }
        }
    }
    //debug(occuped.clone(), free.clone());
    let res1 = checksum(occuped.clone(), free.clone());
    println!();
    println!("{res1}");
}

fn debug(occuped : Vec<(u32,u32,u32)>, free: Vec<(u32, Vec<(u32,u32)>)>) {
    for index in 0..occuped.len() {
        let (id, nbo,_) = occuped[index];
        for _ in 0..nbo {
            print!("{}", id);
        }
        if index == occuped.len()-1 { continue; }
        let (nb, ids) = &free[index];
        for (nb1,id) in ids {
            for i in 0..*nb1 {
                print!("{}", id);
            }
        }
    }
}

fn checksum(occuped : Vec<(u32,u32,u32)>, free: Vec<(u32, Vec<(u32,u32)>)>) -> u64 {
    let mut res = 0u64;
    println!();
    let mut i = 0;
    for index in 0..occuped.len() {
        let (id, nbo,nbsave) = occuped[index];
        if nbo == 0 { i+=nbsave as u64}
        for _ in 0..nbo {
            res += i * id as u64;
            i+=1;
        }
        if index == occuped.len()-1 { continue; }
        let (a, ids) = &free[index];
        for (nb1,id) in ids {
            for _ in 0..*nb1 {
                res += i * *id as u64;
                i+=1;
            }
        }
        if *a != 0 {
            i+= *a as u64;
        }
    }
    res
}