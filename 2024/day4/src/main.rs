use std::{collections::VecDeque, fs};
const XMAS : [char;4] = ['X','M','A','S'];
const XMAS_REV : [char;4] = ['S','A','M','X'];

fn check_vertical_up(map : &mut Vec<Vec<char>> , i: usize, j: usize,pmap : &mut Vec<Vec<char>>) -> bool {
    let mut isxmas = true;
    for nb in 0..4 {
        if let Some(to) = map.get(i+nb) {
            if XMAS[nb] != to[j] {
                isxmas = false;
                break;
            }
        } else {
            isxmas = false;
            break;
        }
    }
    if isxmas {
        for nb in 0..4 {
            pmap[i+nb][j] = XMAS[nb];
        }
    }
    if !isxmas {
        isxmas = true;
        for nb in 0..4 {
            if let Some(to) = map.get(i+nb) {
                if XMAS_REV[nb] != to[j] {
                    isxmas = false;
                    break;
                }
            } else {
                isxmas = false;
                break;
            }
        }
    if isxmas {
        for nb in 0..4 {
            pmap[i+nb][j] = XMAS_REV[nb];
        }
    }
    }
    
    isxmas
}
fn check_horizontal_up(map : &mut Vec<Vec<char>> , i: usize, j: usize,pmap : &mut Vec<Vec<char>>) -> bool {
    let mut isxmas = true;
    for nb in 0..4 {
        if let Some(to) = map[i].get(j+nb) {
            if XMAS[nb] != *to {
                isxmas = false;
                break;
            }
        } else {
            isxmas = false;
            break;
        }
    }
    if isxmas {
        for nb in 0..4 {
            pmap[i][j+nb] = XMAS[nb];
        }
    }
    if !isxmas {
        isxmas = true;
        for nb in 0..4 {
            if let Some(to) = map[i].get(j+nb) {
                if XMAS_REV[nb] != *to {
                    isxmas = false;
                    break;
                }
            } else {
                isxmas = false;
                break;
            }
        }
    if isxmas {
        for nb in 0..4 {
            pmap[i][j+nb] = XMAS_REV[nb];
        }
    }
    }

    isxmas
}
fn check_diag(map : &mut Vec<Vec<char>> , i: usize, j: usize,pmap : &mut Vec<Vec<char>>) -> bool {
    let mut isxmas = true;
    for nb in 0..4 {
        if let Some(to) = map.get(i+nb) {
            if let Some(re)= to.get(j+nb) {
                if XMAS[nb] != *re {
                    isxmas = false;
                    break;
                }
            }
            else { isxmas = false; break;}
        } else { isxmas = false; break; }
    }
    if isxmas {
        for nb in 0..4 {
            pmap[i+nb][j+nb] = XMAS[nb];
        }
    }
    if !isxmas {
        isxmas = true;
        for nb in 0..4 {
            if let Some(to) = map.get(i+nb) {
                if let Some(re)= to.get(j+nb) {
                    if XMAS_REV[nb] != *re {
                        isxmas = false;
                        break;
                    }
                } else { isxmas = false; break; }
            } else { isxmas = false; break; }
        }
    if isxmas {
        for nb in 0..4 {
            pmap[i+nb][j+nb] = XMAS_REV[nb];
        }
    }
    }

    isxmas
}
fn check_diag_left(map : &mut Vec<Vec<char>> , i: usize, j: usize,pmap : &mut Vec<Vec<char>>) -> bool {
    let mut isxmas = true;
    for nb in 0..4 {
        //if i.checked_sub(nb).is_none() { isxmas = false; break;}
        if let Some(to) = map.get(i+nb as usize) {
            if j.checked_sub(nb).is_none() { isxmas = false; break;}
            if let Some(re)= to.get(j-nb as usize) {
                if XMAS[nb] != *re {
                    isxmas = false;
                    break;
                }
            }
            else { isxmas = false; break;}
        } else { isxmas = false; break; }
    }
    if isxmas {
        for nb in 0..4 {
            pmap[i+nb][j-nb] = XMAS[nb];
        }
    }
    if !isxmas {
        isxmas = true;
        for nb in 0..4 {
            //if i.checked_sub(nb).is_none() { isxmas = false; break;}
            if let Some(to) = map.get(i+nb) {
                if j.checked_sub(nb).is_none() { isxmas = false; break;}
                if let Some(re)= to.get(j-nb) {
                    if XMAS_REV[nb] != *re {
                        isxmas = false;
                        break;
                    }
                } else { isxmas = false; break; }
            } else { isxmas = false; break; }
        }
    if isxmas {
        for nb in 0..4 {
            pmap[i+nb][j-nb] = XMAS_REV[nb];
        }
    }
    }

    isxmas
}
fn check_xmas(map : &mut Vec<Vec<char>>, i:usize, j: usize) -> bool {
    if i == 0 || i == map.len()-1 { return false;}
    if j == 0 || j == map[0].len()-1 { return false;}
    let to_test = [(i-1,j-1), (i-1,j+1), (i+1,j+1), (i+1,j-1)];
    let mut xmass= VecDeque::from(['M','M','S','S']);
    let mut pp = Vec::new();
    for (ix,jx) in to_test {
        pp.push(map[ix][jx]);
    }
    //println!("BEGIN");
    for _ in 0..4 {
        let oo = VecDeque::from(pp.clone());
        //println!("CMP");
        //println!("{:?}", xmass);
        //println!("{:?}", oo);
        if xmass.eq(&oo) {
            //println!("TRUE");
            return true;
        }
        xmass.rotate_left(1);
    }
    
    return false;
}
fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    let mut map : Vec<Vec<char>> = Vec::new();
    let mut nb_xmas = 0;
    let mut ligne_len = 0;
    for line in input.lines() {
        map.push(line.chars().collect());
        ligne_len = line.len();
    }
    let mut pmap = vec![vec!['.';ligne_len];map.len()];
    for i in 0..map.len() {
        for j in 0..ligne_len {
            if check_vertical_up(&mut map, i, j,&mut pmap) {
                nb_xmas += 1;
            }
            if check_horizontal_up(&mut map, i, j,&mut pmap) {
                nb_xmas+=1;
            }
            if check_diag(&mut map, i, j,&mut pmap) {
                nb_xmas+=1;
            }
            if check_diag_left(&mut map, i, j,&mut pmap) {
                nb_xmas+=1;
            }
        }
    }
    println!("{}", nb_xmas);
    let mut pmap = vec![vec!['.';ligne_len];map.len()];
    let mut nb_xmas2 = 0;
    for i in 0..map.len() {
        for j in 0..ligne_len {
            if map[i][j] == 'A' {
                if check_xmas(&mut map, i, j) {
                    nb_xmas2+=1;
                    pmap[i][j] = 'A';
                }
            }
        }
    }
    println!("{}", nb_xmas2);
    //printpmap(&mut pmap);
}

fn printpmap(pmap : &mut Vec<Vec<char>>) {
    println!();
    for line in pmap {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}