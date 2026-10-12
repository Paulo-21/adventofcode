use std::fs;
const CARDINAL_8DIR: [(i32, i32); 8] = [
    (0, -1),
    (0, 1),
    (1, 0),
    (-1, 0),
    (-1, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
];
fn p1(pathname: &str) {
    let input = fs::read_to_string(pathname).unwrap();
    let mut res = 0;
    let mut mat: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        mat.push(line.chars().collect());
    }
    for y in 0..mat.len() as i32 {
        for x in 0..mat[0].len() as i32 {
            let mut nb_rolls_paper = 0;
            if mat[y as usize][x as usize] != '@' {
                continue;
            }
            for (x_card, y_card) in CARDINAL_8DIR {
                let new_x = x + x_card;
                let new_y = y + y_card;
                if new_y < 0
                    || new_y >= mat.len() as i32
                    || new_x < 0
                    || new_x >= mat[0].len() as i32
                {
                    continue;
                }
                if mat[new_y as usize][new_x as usize] == '@' {
                    nb_rolls_paper += 1;
                }
            }
            if nb_rolls_paper < 4 {
                res += 1;
            }
        }
    }

    println!("{res}");
}
fn p2(pathname: &str) {
    let input = fs::read_to_string(pathname).unwrap();
    let mut res = 0;
    let mut mat: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        mat.push(line.chars().collect());
    }
    let mut pos_to_remove: Vec<(usize, usize)> = Vec::new();
    loop {
        for y in 0..mat.len() as i32 {
            for x in 0..mat[0].len() as i32 {
                let mut nb_rolls_paper = 0;
                if mat[y as usize][x as usize] != '@' {
                    continue;
                }
                for (x_card, y_card) in CARDINAL_8DIR {
                    let new_x = x + x_card;
                    let new_y = y + y_card;
                    if new_y < 0
                        || new_y >= mat.len() as i32
                        || new_x < 0
                        || new_x >= mat[0].len() as i32
                    {
                        continue;
                    }
                    if mat[new_y as usize][new_x as usize] == '@' {
                        nb_rolls_paper += 1;
                    }
                }
                if nb_rolls_paper < 4 {
                    pos_to_remove.push((x as usize, y as usize));
                    res += 1;
                }
            }
        }
        if pos_to_remove.is_empty() {
            break;
        } else {
            for (x, y) in pos_to_remove.iter_mut() {
                mat[*y][*x] = '.';
            }
            pos_to_remove.clear();
        }
    }

    println!("{res}");
}
fn main() {
    let pathname = "input.txt";
    let pathname = "exemple.txt";
    p1(pathname);
    p2(pathname);
    //1667 TOO Hight
}
