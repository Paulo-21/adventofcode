use std::str::Lines;

use crate::input::get_input;

mod input;
fn p1() {
    let input = get_input();
    let a = input.lines();
    let mut most_tot = 0;
    let mut current_tot = 0;
    for b in a {
        if b.is_empty() {
            if most_tot < current_tot {
                most_tot = current_tot;
            }
            current_tot = 0;
            continue;
        }
        current_tot += b.parse::<u32>().unwrap();
    }
    println!("{most_tot}");
}
fn p2() {
    let input = get_input();
    let a = input.lines();
    let mut most_tot1 = 0;
    let mut most_tot2 = 0;
    let mut most_tot3 = 0;
    let mut current_tot = 0;
    for b in a {
        if b.is_empty() {
            if most_tot1 < current_tot {
                most_tot1 = current_tot;
                
            }
            else if most_tot2 < current_tot {
                most_tot2 = current_tot;
                
            }
            else if most_tot3 < current_tot {
                most_tot3 = current_tot;
                
            }
            current_tot = 0;
            continue;
        }
        current_tot += b.parse::<u32>().unwrap();
    }
    println!("{}", most_tot1+most_tot2+most_tot3);
}
fn main() {
    p2();
}
