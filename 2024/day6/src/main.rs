use std::{collections::BTreeSet, fs};
use owo_colors::OwoColorize;
#[derive(Debug,Clone, Copy,PartialEq, Eq, Hash, PartialOrd, Ord)]
struct GardePos {
    i:i32,
    j:i32,
    d:D
}
#[derive(Debug,PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
enum D {
    U,D,L,R,
}
fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    //let input = fs::read_to_string("exemple2").unwrap();
    let mut rotate = [D::U,D::R,D::D,D::L];
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut garde_pos = GardePos{i:0, j:0, d:D::U};
    for (i,line) in input.lines().enumerate() {
        map.push(line.chars().collect());
        if let Some(pos) = line.chars().position(|c| c == '^' || c == 'v' || c == '<' || c == '>' ) {
            let d = match map[i][pos] {
                '^' => D::U,
                'v' => D::D,
                '<' => D::L,
                '>' => D::R,
                _ => panic!("imp"),
            };
            garde_pos = GardePos{i: i as i32,j:pos as i32, d};
        }
    }
    let garde_save = garde_pos;
    let imax = map.len() as i32;
    let jmax = map[0].len() as i32;
    let mut marque: Vec<Vec<bool>> = vec![vec![false;map[0].len()];map.len()];

    let rd = rotate.iter().position(|d| *d == garde_pos.d).unwrap();
    rotate.rotate_left(rd);
    let mut done = false;
    while !done {
        if is_outside(&garde_pos, (0,0), imax, jmax) { break; }
        marque[garde_pos.i as usize][garde_pos.j as usize] = true;
        for _ in 0..4 {
            let pas = match garde_pos.d {
                D::U => (-1,0), D::D => (1,0), D::L => (0,-1), D::R => (0,1),
            };
            if !is_outside(&garde_pos, pas, imax, jmax) && map[(garde_pos.i+pas.0) as usize][(garde_pos.j+pas.1) as usize] != '#' {
                garde_pos.i += pas.0;
                garde_pos.j += pas.1;
                break;
            } else if is_outside(&garde_pos, pas, imax, jmax) { done = true; break;}
            rotate.rotate_left(1);
            garde_pos.d = rotate[0];
        }
    }
    let mut nb_mark = 0;
    for l in &marque { for m in l { if *m { nb_mark +=1; } } }
    println!("{nb_mark}");
//----------------------P2------------------------
    let mut nb_loop = 0;
    
    for (i,line) in marque.iter().enumerate() {
        for (j,m) in line.iter().enumerate() {
            if *m {
                if garde_save.i == i as i32 && garde_save.j == j as i32 { continue; }
                map[i][j] = '#';
                if simulation(&map, garde_save) {
                    nb_loop+=1;
                }
                map[i][j] = '.';
            }
        }
    }
    
    /*for l in map {
        for k in l {
            if k == '#' { print!("#"); }
            else if k == '^' || k == 'v' || k == '<' || k == '>'  { print!("{}",k); }
            //else if choosen_obs.get(&(i as i32,j as i32)).is_some() { print!("o") }
            else { print!("."); }
        }
        println!();
    }*/

    println!("{nb_loop}"); 
}

fn simulation(map : &[Vec<char>], mut garde_pos : GardePos,) -> bool {//1686 to hight be same as other
    //let done = false;//1592 HIGHT
    let imax = map.len() as i32;
    let jmax = map[0].len() as i32;
    let mut rotate = [D::U,D::R,D::D,D::L];
    let rd = rotate.iter().position(|d| *d == garde_pos.d).unwrap();
    rotate.rotate_left(rd);
    
    let mut detectloop: BTreeSet<GardePos> = BTreeSet::new();
    loop {
        if is_outside(&garde_pos, (0,0), imax, jmax) { break; }
        if detectloop.contains(&garde_pos) {
            /*println!("-----------------------------------------------");
            let mut mappr = map.clone();
            for pt in detectloop {
                match pt.d {
                    D::U => {mappr[pt.i as usize][pt.j as usize] = '^';}
                    D::D => {mappr[pt.i as usize][pt.j as usize] = 'v'}
                    D::L => {mappr[pt.i as usize][pt.j as usize] = '<'}
                    D::R => {mappr[pt.i as usize][pt.j as usize] = '>'}
                }
            }
            mappr[garde_pos.i as usize][garde_pos.j as usize] ='L';
            mappr[first.i as usize][first.j as usize] ='F';
            mappr[o.0 as usize][o.1 as usize] ='O';
            for (i,l) in mappr.iter().enumerate() {
                for (j,k) in l.iter().enumerate() {
                    if *k == '.' { print!("{}", *k);}
                    else if *k == 'F' { print!("{}", (*k).cyan());}
                    else if *k == 'O' { print!("{}", (*k).green());}
                    else if *k == '#' { print!("{}", (*k).blue());}
                    else { print!("{}", (*k).red());}
                }
                println!();
            }
            println!("---------------------------------");*/
            return true;
        }
        detectloop.insert(garde_pos);
        //println!("--------------------");
        //println!("first : {:?}", garde_pos);
        for _blockiter in 0..4 {
            let pas = match garde_pos.d {
                D::U => (-1,0), D::D => (1,0), D::L => (0,-1), D::R => (0,1),
            };
            if !is_outside(&garde_pos, pas, imax, jmax) && map[(garde_pos.i+pas.0) as usize][(garde_pos.j+pas.1) as usize] != '#' {
                garde_pos.i += pas.0;
                garde_pos.j += pas.1;
                break;
            } else if is_outside(&garde_pos, pas, imax, jmax) { return false; }
            rotate.rotate_left(1);
            garde_pos.d = rotate[0];
        }
        
    }
    false
}

fn is_outside (garde : &GardePos, pas : (i32,i32), imax: i32, jmax: i32) -> bool {
    garde.i+pas.0 < 0 || garde.i+pas.0 >= imax || garde.j+pas.1 < 0 || garde.j+pas.1 >= jmax
}
fn _print_itmap(map : Vec<Vec<char>>, itmap : Vec<Vec<bool>>) {
    for (i,l) in map.iter().enumerate() {
        for (j,k) in l.iter().enumerate() {
            if *k == '#' { print!("#"); }
            else if itmap[i][j] { print!("X")}
            else { print!("."); }
        }
        println!();
    }
}