use std::fs;
use std::collections::BTreeMap;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    //let input = fs::read_to_string("test").unwrap();
    let numbers : Vec<u64> = input.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
    let mut cache: BTreeMap<(u64,u64), u64> = BTreeMap::new();
    let nb_state1 = 25;
    let nb_state2 = 75;
    let mut res1 = 0;
    let mut res2 = 0;
    
    for n in numbers {
        res1 += rec(n, &mut cache, nb_state1 as u64);
        res2 += rec(n, &mut cache, nb_state2 as u64);
    }
    println!("{res1}");
    println!("{res2}");
}
fn rec(num : u64, cache : &mut BTreeMap<(u64,u64),u64>, mut blinking : u64) -> u64 {
    if blinking == 0 { return 1; }
    if cache.contains_key(&(num,blinking)) {
        return *cache.get(&(num, blinking)).unwrap();
    }
    blinking -=1;
    let mut sol = 0;
    if num == 0 { sol = rec(1, cache, blinking); }
    else if (num.ilog10() + 1) % 2 == 0 {
        let digitcount = num.ilog10() + 1;
        let first = num/(10u64.pow(digitcount/2));
        sol += rec(first, cache,blinking);
        sol += rec(num - (first*10u64.pow(digitcount/2)), cache,blinking);
    }
    else {
        sol = rec(num*2024, cache,blinking);
    }
    cache.insert((num, blinking+1), sol);
    return sol;
}