use std::fs;
const DIR : [(i64,i64);4] = [(-1,0),(1,0),(0,-1),(0,1)];
fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input = fs::read_to_string("exemple").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        map.push(line.chars().collect());
    }
    let mut res1 = 0;
    let mut visited: Vec<Vec<bool>> = vec![vec![false;map[0].len()];map.len()];
    for i in 0..visited.len() {
        for j in 0..visited[0].len() {
            let regiontype = map[i][j];
            if !visited[i][j] {
                let (perimeter, area) = rec(&map, &mut visited, regiontype, (i as i64, j as i64));
                res1 += perimeter * area;
            }
        }
    }
    println!("{res1}");
}
fn rec(map : &Vec<Vec<char>>, visited : &mut Vec<Vec<bool>>, rtype : char, current : (i64,i64)) -> (u64,u64) {
    visited[current.0 as usize][current.1 as usize] = true;
    let mut area = 0;
    let mut perimeter = 0;
    let mut alone = 4;
    for d in DIR {
        let new = (current.0 + d.0, current.1 + d.1);
        if !is_outside(map, new) && map[new.0 as usize][new.1 as usize] == rtype{
            if !visited[new.0 as usize][new.1 as usize] {
                let (p, a) = rec(map, visited, rtype, new);
                perimeter += p;
                area += a;
            }
            alone -=1;
        }
    }
    perimeter+=alone;
    return (perimeter, area+1);
}
fn is_outside(map : &Vec<Vec<char>>, pos : (i64,i64) ) -> bool {
    pos.0 < 0 || pos.1 < 0 || pos.0 >= map.len() as i64 || pos.1 >= map[0].len() as i64
}