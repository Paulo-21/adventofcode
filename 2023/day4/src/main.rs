use std::fs;

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    
    let mut res = 0;
    for line in file.lines() {
        let mut first_match = true;
        let mut line_score = 0;
        let mut s = line.split(':');
        let _ = s.next();
        let n = s.next().unwrap();
        let mut nsplit = n.split('|');
        let winning_number: Vec<&str>= nsplit.next().unwrap().split_ascii_whitespace().collect();
        let my_number = nsplit.next().unwrap().split_ascii_whitespace();
        
        for num in my_number {
            for w in &winning_number {
                if num.parse::<u32>().unwrap() ==  (*w).parse::<u32>().unwrap() {
                    if first_match {
                        first_match =false;
                        line_score = 1;
                    }
                    else {
                        line_score *= 2;
                    }
                }
            }
            
        }
        res += line_score;
    }
    println!("{}", res);
}
fn p2() {
    let file = fs::read_to_string("input").unwrap();
    let mut nb_lignes = 0;
    for c in file.lines() {
        nb_lignes+=1;
    }
    let mut instance = vec![1;nb_lignes];
    let mut res = 0;
    for (index, line) in file.lines().enumerate() {
        let mut line_score = 0;
        let mut s = line.split(':');
        let _ = s.next();
        let n = s.next().unwrap();
        let mut nsplit = n.split('|');
        let winning_number: Vec<&str>= nsplit.next().unwrap().split_ascii_whitespace().collect();
        let my_number = nsplit.next().unwrap().split_ascii_whitespace();
        
        for num in my_number {
            for w in &winning_number {
                if num.parse::<u32>().unwrap() ==  (*w).parse::<u32>().unwrap() {
                    line_score+=1;
                    instance[index+line_score] += 1*instance[index];
                }
            }
        }
    }
    for c in instance {
        res += c;
    }
    println!("{}", res);
}

fn main() {
    //p1();
    p2();
}
