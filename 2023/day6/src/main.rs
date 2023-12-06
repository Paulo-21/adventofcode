use std::fs;


fn p1() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut iter = file.lines();
    let mut times : Vec<&str> = iter.next().unwrap().split_ascii_whitespace().collect();
    let mut distance : Vec<&str> = iter.next().unwrap().split_ascii_whitespace().collect();
    times.remove(0);
    distance.remove(0);
    let timec : Vec<u64> = times.iter().map(|p| p.parse::<u64>().unwrap()).collect();
    let distancec : Vec<u64> = distance.iter().map(|p| p.parse::<u64>().unwrap()).collect();
    eprintln!("{:?}", timec);
    eprintln!("{:?}", distancec);
    let mut res = 1;
    for (i,time) in timec.iter().enumerate() {
        let mut nb = 0;
        for t in 1..*time {
            if t*(time-t) > distancec[i] {
                nb+=1;
            }  
        }
        println!("{}", nb);
        res *= nb;
    }
    println!("{}", res);
}
fn p2() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut iter = file.lines();
    let mut time = iter.next().unwrap().split(":").nth(1).unwrap().to_string();
    time.retain(|c| !c.is_whitespace());
    let mut distance = iter.next().unwrap().split(":").nth(1).unwrap().to_string();
    distance.retain(|c| !c.is_whitespace());
    
    let timec : u64 = time.parse::<u64>().unwrap();
    let distancec : u64 = distance.parse::<u64>().unwrap();
    let mut nb = 0;
    for t in 1..timec {
        if t*(timec-t) > distancec {
            nb+=1;
        }  
    }
    println!("{}", nb);
}


fn main() {
    //p1();
    p2();
}
