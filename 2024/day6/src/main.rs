use std::fs;
struct GardePpos {
    i:i32,
    j:i32,
    d:D
}
#[derive(PartialEq, Clone, Copy)]
enum D {
    U,D,L,R,
}
fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    let mut rotate = [D::U,D::R,D::D,D::L];
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut garde_pos = GardePpos{i:0, j:0, d:D::U};
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
            garde_pos = GardePpos{i: i as i32,j:pos as i32, d};
        }
    }
    let imax = map.len() as i32;
    let jmax = map[0].len() as i32;
    let mut marque: Vec<Vec<bool>> = vec![vec![false;map[0].len()];map.len()];
    let mut i = 0;
    let rd = rotate.iter().position(|d| *d == garde_pos.d).unwrap();
    rotate.rotate_left(rd);
    let mut done = false;
    while !done {
        if is_outside(&garde_pos, (0,0), imax, jmax) { break; }

        marque[garde_pos.i as usize][garde_pos.j as usize] = true;
        
        //rotate.rotate_left(rd);
        for _ in 0..4 {
            let pas = match garde_pos.d {
                D::U => (-1,0),
                D::D => (1,0),
                D::L => (0,-1),
                D::R => (0,1),
            };
            if !is_outside(&garde_pos, pas, imax, jmax) && map[(garde_pos.i+pas.0) as usize][(garde_pos.j+pas.1) as usize] != '#' {
                garde_pos.i += pas.0;
                garde_pos.j += pas.1;
                break;
            } else if is_outside(&garde_pos, pas, imax, jmax) { done = true; break;}
            rotate.rotate_left(1);
            garde_pos.d = rotate[0];
        }
        i+=1;
    }


    let mut nb_mark = 0;
    for l in &marque {
        for m in l {
            if *m { nb_mark +=1; }
        }
    }
    println!("{nb_mark}");
    //print_itmap(map, marque);

}

fn is_outside (garde : &GardePpos, pas : (i32,i32), imax: i32, jmax: i32) -> bool {
    garde.i+pas.0 < 0 || garde.i+pas.0 >= imax || garde.j+pas.1 < 0 || garde.j+pas.1 >= jmax
}
fn print_itmap(map : Vec<Vec<char>>, itmap : Vec<Vec<bool>>) {
    for (i,l) in map.iter().enumerate() {
        for (j,k) in l.iter().enumerate() {
            if *k == '#' { print!("#"); }
            else if itmap[i][j] { print!("X")}
            else { print!("."); }
        }
        println!();
    }
}