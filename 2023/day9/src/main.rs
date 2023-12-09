use std::fs;

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut res = 0;
    for line in file.lines() {
        let numbers :Vec<&str> = line.split_ascii_whitespace().collect();
        let numbers : Vec<i64> = numbers.iter().map(|str| str.parse::<i64>().unwrap()).collect();
        let mut all_n = Vec::new();
        all_n.push(numbers);
        while !all_zero(all_n.last().unwrap()) {
            let mut nv = Vec::with_capacity(all_n.last().unwrap().len()-1);
            let mut i = 0;
            while i < nv.capacity() {
                nv.push(all_n.last().unwrap()[i+1] - all_n.last().unwrap()[i]);
                i+=1;
            }
            all_n.push(nv);
            //println!("{:?}", all_n);
            //println!();
        }
        let mut next_prediction = 0;
        for rev in all_n.iter().rev() {
            next_prediction += rev.last().unwrap();
        }
        res += next_prediction;
    }
    println!("{}", res);
}
fn p2() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut res = 0;
    for line in file.lines() {
        let numbers :Vec<&str> = line.split_ascii_whitespace().collect();
        let numbers : Vec<i64> = numbers.iter().map(|str| str.parse::<i64>().unwrap()).collect();
        let mut all_n = Vec::new();
        all_n.push(numbers);
        while !all_zero(all_n.last().unwrap()) {
            let mut nv = Vec::with_capacity(all_n.last().unwrap().len()-1);
            let mut i = 0;
            while i < nv.capacity() {
                nv.push(all_n.last().unwrap()[i+1] - all_n.last().unwrap()[i]);
                i+=1;
            }
            all_n.push(nv);
            //println!("{:?}", all_n);
            //println!();
        }
        let mut next_prediction = 0;
        for rev in all_n.iter().rev() {
            next_prediction = rev.first().unwrap() - next_prediction;
        }
        res += next_prediction;
    }
    println!("{}", res);
}
fn all_zero (vec : &[i64]) -> bool {
    for v in vec {
        if *v != 0 {
            return false;
        }
    }
    true
}

fn main() {
    //p1();
    p2();
}
