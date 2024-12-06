use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    let mut blank = false;
    let mut res = 0;
    let mut ordering_before: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut incorrect = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            blank = true;
            continue;
        }
        
        if !blank {
            let s : Vec<&str> = line.split('|').collect();
            let before = s[0].parse().unwrap();
            let after  = s[1].parse::<u32>().unwrap();
            ordering_before.entry(before).or_insert(Vec::new());
            ordering_before.get_mut(&before).unwrap().push(after);
        }
        else {
            let mut correctly_sorted = true;
            let mut all = Vec::new();
            let mut h = HashSet::new();
            for number in line.split(',') {
                let n = number.parse::<u32>().unwrap();
                if let Some(to_be_after ) = ordering_before.get(&n) {
                    for k in to_be_after {
                        if h.contains(k) {
                            correctly_sorted = false;
                        }
                    }
                }
                h.insert(n);
                all.push(n);
            }
            if correctly_sorted {
                res += all[all.len()/2];
            } else { incorrect.push(all);}
        }
    }
    println!("{res}");
    let mut res2 = 0;
    for wrong in incorrect {
        let mut ppp = Vec::new();
        let a: HashSet<u32> = HashSet::from_iter(wrong.clone());
        for n in 0..wrong.len() {
            if let Some(to_be_after ) = ordering_before.get(&wrong[n]) {
                let b: HashSet<u32> = HashSet::from_iter(to_be_after.clone());
                let inter = a.intersection(&b);
                let nb_inter = inter.count();
                ppp.push((wrong[n],nb_inter));
            }
        }
        ppp.sort_unstable_by_key(|k| k.1);
        ppp.reverse();
        res2 += ppp[ppp.len()/2].0;
    }
    println!("{res2}");
}