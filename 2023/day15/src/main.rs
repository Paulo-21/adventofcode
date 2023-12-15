use std::{fs, cmp::Ordering};

fn hash (input : &str) -> usize {
    let mut current = 0;
    for c in input.chars() {
        current += c as usize;
        current *= 17;
        current %= 256;
    }
    current
}
fn p1() {
    let mut res = 0;
    let file = fs::read_to_string("input").unwrap();
    for line in file.split(',') {
        res += hash(line);
    }
    println!("{}", res);
}
fn print_boxes (b : &Vec<Vec<(String, usize)>>) {
    for (i, a) in b.iter().enumerate() {
        if !a.is_empty() {
            println!("Boxes {} : {:?} ",i , a);
        }
    }
    println!();
}
fn p2() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut boxes = vec![Vec::<(String,usize)>::new(); 256];
    for line in file.split(',') {
        let mut label = "";
        let mut op = 'u';
        let mut lenses_type = 0;
        if line.contains('-') {
            label = &line[..line.len()-1];
            op = '-';
        }
        else {
            label = &line[..line.len()-2];
            op = '=';
            lenses_type = line.chars().last().unwrap() as usize -48;
        }
        let hash = hash(label);
        let mut already_in = false;
        let mut index_already_in = 0;
        for (i , one) in (&boxes[hash]).iter().enumerate() {
            if one.0.as_str() == label {
                already_in = true;
                index_already_in = i;
                break;
            }
        }
        if already_in {
            if op == '=' {
                boxes[hash][index_already_in].1 = lenses_type;
            }
            else {
                boxes[hash].remove(index_already_in);
            }
        }
        else {
            if op == '=' {
                boxes[hash].push((label.to_string(), lenses_type));
            }
        }
        //print_boxes(&boxes);
    }
    let mut res = 0;
    for (i, onebox) in boxes.iter().enumerate() {
        for (k, lenses) in onebox.iter().enumerate() {
            res += (i+1) * (k+1) * lenses.1;
        }
    }
    println!("{}", res);
}

fn main() {
    //p1();
    p2();
}
