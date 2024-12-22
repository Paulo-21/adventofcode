use std::fs;

fn main() {
    let mut res1 = 0;
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple_mini").unwrap();
    for line in input.lines() {
        let number : u64 = line.parse().unwrap();
        res1 += process1(number);
    }
    println!("{res1}");
}

fn process1(mut number : u64) -> u64 {
    #[allow(unused_assignments)]
    let mut lsecret = 0;
    for _ in 0..2000 {
        lsecret = number;
        number*=64;
        number = number ^ lsecret;
        number %= 16777216;
        lsecret = number;
        number /= 32;
        number = number ^ lsecret;
        number %= 16777216;
        lsecret = number;
        number *= 2048;
        number = number ^ lsecret;
        number %= 16777216;
    }
    number
}
