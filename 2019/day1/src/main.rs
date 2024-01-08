use std::fs;

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    let mut res = 0;
    for line in file.lines() {
        let mass : u64 = line.parse().unwrap();
        res += (mass/3)-2;
    }
    println!("{res}");
}
fn p2() {
    let file = fs::read_to_string("input").unwrap();
    let mut res = 0;
    for line in file.lines() {
        let mut mass : i64 = line.parse().unwrap();
        while (mass/3) -2 > 0{
            mass = (mass/3) -2;
            res += mass;
        }
    }
    println!("{res}");
}

fn main() {
    p1();
    p2();
}
