use std::fs;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    let mut res = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let s:Vec<&str> = line.split(':').collect();
        let op_resultat :u64 = s[0].parse().unwrap();
        let numbers:Vec<&str> = s[1].split_ascii_whitespace().collect();
        let nums: Vec<u64> = numbers.iter().map(|s| s.parse::<u64>().unwrap()).collect();
        if find_seq(&nums[1..], nums[0], op_resultat) {
            res += op_resultat;
        }
        
        if find_seq2(&nums[1..], nums[0], op_resultat, false) {
            res2 += op_resultat;
        }       
    }
    println!("{res}");
    println!("{}", res2);
}
fn find_seq(seq : &[u64], acc : u64, res : u64) -> bool {
    if seq.is_empty() { return acc == res }
    if find_seq(&seq[1..], acc*seq[0], res) { return true; } 
    return find_seq(&seq[1..], acc+seq[0], res);
}
fn find_seq2(seq : &[u64], acc : u64, res : u64, print:bool) -> bool {
    if seq.is_empty() { return acc == res }
    if find_seq2(&seq[1..], acc*seq[0], res, print) { return true; }
    if find_seq2(&seq[1..], acc+seq[0], res,print) { return true; }
    let digitcount = seq[0].ilog10() + 1;
    return find_seq2(&seq[1..], (acc*10u64.pow(digitcount))+seq[0], res,print);
}
