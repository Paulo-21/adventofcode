use std::{fs, time::Instant};

fn p1(input: &String) {
    println!("Partie 1");

    let mut interval_list = Vec::new();
    let mut interval_mode = true;
    let mut res = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            interval_mode = false;
            continue;
        }
        if interval_mode {
            let mut s = line.split('-');
            let lower: u64 = s.next().unwrap().parse().unwrap();
            let upper: u64 = s.next().unwrap().parse().unwrap();
            interval_list.push((lower, upper));
        } else {
            let number: u64 = line.parse().unwrap();
            let mut inside = false;
            for (l, u) in interval_list.iter() {
                if *l <= number && number <= *u {
                    inside = true;
                    break;
                }
            }
            if inside {
                res += 1;
            }
        }
    }
    println!("{res}");
}
fn p2(input: &String) {
    println!("Partie 2");
    let mut res = 0;
    let mut interval_list: Vec<(u64, u64)> = Vec::new();

    // 1. Lire TOUS les intervalles en premier
    for line in input.lines() {
        if line.trim().is_empty() {
            break;
        }
        let mut s = line.split('-');
        let lower: u64 = s.next().unwrap().parse().unwrap();
        let upper: u64 = s.next().unwrap().parse().unwrap();
        interval_list.push((lower, upper));
    }

    interval_list.sort_by_key(|&(lower, _)| lower);

    let mut merged_intervals: Vec<(u64, u64)> = Vec::new();
    if let Some(&first) = interval_list.first() {
        let mut current = first;

        for &(lower, upper) in interval_list.iter().skip(1) {
            if lower <= current.1 {
                current.1 = current.1.max(upper);
            } else {
                merged_intervals.push(current);
                current = (lower, upper);
            }
        }
        merged_intervals.push(current);
    }

    for (l, u) in merged_intervals {
        res += u - l + 1;
    }
    println!("{res}");
}

fn main() {
    let pathname = "input.txt";
    //let pathname = "exemple.txt";
    let input = fs::read_to_string(pathname).unwrap();
    p1(&input);
    let start = Instant::now();
    p2(&input);
    // Low 294907584187021
    // Hight 1112740630545400
    println!("{} ms", start.elapsed().as_millis());
}
