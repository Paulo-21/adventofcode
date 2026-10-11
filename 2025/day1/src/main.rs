use std::{fs, ops::Div};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    //let input = fs::read_to_string("exemple2.txt").unwrap();
    let mut rotate = 50;
    let mut password = 0;
    for line in input.lines() {
        let val = line[1..].parse::<i32>().unwrap();

        if line.starts_with('L') {
            rotate -= val % 100;
            if rotate < 0 {
                rotate = 100 - (rotate % 100).abs();
            }
        } else if line.starts_with('R') {
            rotate += val % 100;
            if rotate > 99 {
                rotate = rotate % 100;
            }
        }
        if rotate == 0 {
            password += 1;
        }
    }
    //println!("{password}");
    let mut password2 = 0;
    rotate = 50;
    //8096 TOO HIGHT
    //2405 NO
    for line in input.lines() {
        let mut val = line[1..].parse::<i32>().unwrap();
        password2 += val.div(100);
        val %= 100;
        println!("{val}");
        if line.starts_with('L') && rotate != 0 && val != 0 {
            if val >= rotate {
                password2 += 1;
                //print!("0 : {rotate} {line}");
            }
        } else if line.starts_with('R') && rotate != 0 && val != 0 {
            if val >= 100 - rotate {
                password2 += 1;
                //print!("0 : {rotate}, {line}");
            }
        }
        if line.starts_with('L') {
            rotate -= val;
        } else if line.starts_with('R') {
            rotate += val;
        }
        rotate = rotate.rem_euclid(100);
        println!(" {line} -> {}", rotate);
    }
    println!("{password2}");
}
//Low 4120
//Hight 6686
//1893 NO
// 5985 NO
