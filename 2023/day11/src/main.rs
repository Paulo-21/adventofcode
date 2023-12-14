use std::{fs, cmp::{min, max}};


fn p1_2(len_of_empty : i64) {
    let file = fs::read_to_string("input2").unwrap();
    let mut empty_column_index = vec![true;140];
    let mut empty_row_index = vec![true;140];
    let mut galaxy_coord = Vec::new();
    for (i, line) in file.lines().enumerate() {
        for (k, c) in line.chars().enumerate() {
            if c == '#' {
                galaxy_coord.push((i as i64,k as i64));
                empty_row_index[i] = false;
                empty_column_index[k] = false;
            }
        }
    }
    let mut res = 0i64;
    while !galaxy_coord.is_empty() {
        let first = galaxy_coord.pop().unwrap();
        for galaxy in &galaxy_coord {
            res += (galaxy.0 - first.0).abs();
            res += (galaxy.1 - first.1).abs();
            for i in min(galaxy.0, first.0)..max(galaxy.0, first.0) {
                if empty_row_index[i as usize] {
                    res +=len_of_empty-1;
                }
            }
            for i in min(galaxy.1, first.1)..max(galaxy.1, first.1) {
                if empty_column_index[i as usize] {
                    res +=len_of_empty-1;
                }
            }
        }
    }

    println!("{}", res);
}

fn main() {
    //p1_2(2);
    p1_2(1000000);
}
 