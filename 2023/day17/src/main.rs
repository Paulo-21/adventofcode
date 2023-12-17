use std::{fs, usize, vec, default, time::Instant};
use priority_queue::PriorityQueue;
use hashbrown::hash_map::DefaultHashBuilder;
use colored::Colorize;
use fxhash::FxBuildHasher;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq, Default)]
struct CityBlock {
    y : usize,
    x : usize,
    cout : i32,
    heuristic : i32, 
    consecutive : u8,
    dir : Direction,
}

fn heuristic(one : (usize, usize), two : (usize, usize)) -> i32 {
    (one.0 as i32 - two.0 as i32).abs() + (one.1 as i32 - two.1 as i32).abs()
}

fn get_neighbour(map : &Vec<Vec<i32>>, from : &CityBlock, h:usize, w:usize) -> Vec<CityBlock> {
    let mut v = Vec::with_capacity(4); 
    let mut maximum = false;
    if (from.dir == Direction::Up || from.dir == Direction::Down) && from.consecutive == 3 {//(from.consecutive < 4 || from.consecutive >= 10) {
        maximum = true;
    }
    if (from.dir == Direction::Right || from.dir == Direction::Left) &&from.consecutive == 3 { //(from.consecutive < 4  || from.consecutive >= 10) {
        maximum = true;
    }
    
    if let Some(x) = from.y.checked_sub(1) {
        
        let mut consecutive = from.consecutive+1;
        let mut can_push = false;
        if from.dir == Direction::Up && !maximum {
            can_push = true;
        }
        else if from.dir != Direction::Up {
            consecutive = 1;
            can_push = true;
        }
        if from.dir == Direction::Down { can_push = false;}
        if can_push {
            v.push(CityBlock{
                y : from.y-1, x : from.x, dir : Direction::Up, consecutive : consecutive,
                cout : from.cout + map[from.y-1][from.x], heuristic : 0
            });
        }
    }
    if let Some(x) = from.x.checked_sub(1) {
        let mut consecutive = from.consecutive+1;
        let mut can_push = false;
        if from.dir == Direction::Left && !maximum {
            can_push = true;
        }
        else if from.dir != Direction::Left {
            consecutive = 1;
            can_push = true;
        }
        if from.dir == Direction::Right { can_push = false;}
        if can_push {
            v.push(CityBlock{
                y : from.y, x : from.x-1, dir : Direction::Left, consecutive : consecutive,
                cout : from.cout + map[from.y][from.x-1], heuristic : 0
            });
        }
    }
    if from.y+1 < h {
        let mut consecutive = from.consecutive+1;
        let mut can_push = true;
        if from.dir == Direction::Down && maximum {
            can_push = false;
        }
        if from.dir != Direction::Down {
            consecutive = 1;
        }
        if from.dir == Direction::Up { can_push = false;}
        if can_push {
            v.push(CityBlock{
                y : from.y+1, x : from.x, dir : Direction::Down, consecutive : consecutive,
                cout : from.cout + map[from.y+1][from.x], heuristic : 0
            });
        }
    }
    if from.x +1 < w {
        let mut consecutive = from.consecutive+1;
        let mut can_push = false;
        if from.dir == Direction::Right && !maximum {
            can_push = true;
        }
        else if from.dir != Direction::Right {
            consecutive = 1;
            can_push = true;
        }
        if from.dir == Direction::Left { can_push = false;}
        if can_push {
            v.push(CityBlock{
                y : from.y, x : from.x+1, dir : Direction::Right, consecutive : consecutive,
                cout : from.cout + map[from.y][from.x+1], heuristic : 0
            });
        }
    }
    v 
}
fn get_neighbour2(map : &Vec<Vec<i32>>, from : &CityBlock, h:usize, w:usize) -> Vec<CityBlock> {
    let mut v = Vec::with_capacity(4); 
    let mut maximum = false;
    
    if (from.dir == Direction::Up || from.dir == Direction::Down) && from.consecutive == 10 {//(from.consecutive < 4 || from.consecutive >= 10) {
        maximum = true;
    }
    if (from.dir == Direction::Right || from.dir == Direction::Left) && from.consecutive == 10 { //(from.consecutive < 4  || from.consecutive >= 10) {
        maximum = true;
    }
    
    if let Some(x) = from.y.checked_sub(1) {
        
        let mut consecutive = from.consecutive + 1;
        let mut can_push = false;
        if from.dir == Direction::Up && !maximum {
            can_push = true;
        }
        else if from.dir != Direction::Up {
            consecutive = 1;
            can_push = true;
        }
        if from.dir == Direction::Down || (from.dir != Direction::Up && from.consecutive < 4) { can_push = false;}
        if can_push {
            v.push(CityBlock{
                y : from.y-1, x : from.x, dir : Direction::Up, consecutive : consecutive,
                cout : from.cout + map[from.y-1][from.x], heuristic : 0
            });
        }
    }
    if let Some(x) = from.x.checked_sub(1) {
        let mut consecutive = from.consecutive+1;
        let mut can_push = false;
        if from.dir == Direction::Left && !maximum {
            can_push = true;
        }
        else if from.dir != Direction::Left {
            consecutive = 1;
            can_push = true;
        }
        if from.dir == Direction::Right || (from.dir != Direction::Left && from.consecutive < 4){ can_push = false;}
        if can_push {
            v.push(CityBlock{
                y : from.y, x : from.x-1, dir : Direction::Left, consecutive : consecutive,
                cout : from.cout + map[from.y][from.x-1], heuristic : 0
            });
        }
    }
    if from.y+1 < h {
        let mut consecutive = from.consecutive+1;
        let mut can_push = true;
        if from.dir == Direction::Down && maximum {
            can_push = false;
        }
        if from.dir != Direction::Down {
            consecutive = 1;
        }
        if from.dir == Direction::Up || (from.dir != Direction::Down && from.consecutive < 4) { can_push = false;}
        if can_push {
            v.push(CityBlock{
                y : from.y+1, x : from.x, dir : Direction::Down, consecutive : consecutive,
                cout : from.cout + map[from.y+1][from.x], heuristic : 0
            });
        }
    }
    if from.x +1 < w {
        let mut consecutive = from.consecutive+1;
        let mut can_push = false;
        if from.dir == Direction::Right && !maximum {
            can_push = true;
        }
        else if from.dir != Direction::Right {
            consecutive = 1;
            can_push = true;
        }
        if from.dir == Direction::Left|| (from.dir != Direction::Right && from.consecutive < 4) { can_push = false;}
        if can_push {
            v.push(CityBlock{
                y : from.y, x : from.x+1, dir : Direction::Right, consecutive : consecutive,
                cout : from.cout + map[from.y][from.x+1], heuristic : 0
            });
        }
    }
    v 
}

fn p1() {
    //let file = fs::read_to_string("example_little").unwrap();
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut map : Vec<Vec<i32>> = Vec::new();
    for line in file.lines() {
        map.push(line.chars().map(|c| c as i32 -48).collect());
    }
    let h = map.len();
    let w = map[0].len();
    
    let mut pred : Vec<Vec<Vec<(usize, usize, usize)>>> = vec![vec![vec![(usize::MAX, usize::MAX, usize::MAX);4];w]; h];
    let mut visited : Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![i32::MAX;11];4];w]; h];
    let exit_pos = ((map.len()-1), map[0].len()-1);
    //let mut pq = PriorityQueue::<CityBlock, i32, DefaultHashBuilder>::with_default_hasher();
    let mut pq = PriorityQueue::<CityBlock, i32, FxBuildHasher>::with_default_hasher();
    pq.push(CityBlock { y:0, x:0, consecutive:0 , dir : Direction::Right, cout : 0, heuristic: 0}, 0);
    let start = Instant::now();
    let mut finisher =  CityBlock::default();
    loop  {
        if pq.is_empty() { 
            println!("EMPTY");
            break;
        }
        let (current, _) = pq.pop().unwrap();
        //println!("{:?}", current);
        if current.x == exit_pos.1 && current.y == exit_pos.0 { 
            println!("FIND {}", map[exit_pos.0][exit_pos.1]);
            
            finisher = current;
            break; 
        }
        
        //let neighbour = get_neighbour(&map, &current, h, w);
        let neighbour = get_neighbour2(&map, &current, h, w);
        //println!("Nei : {:?}", neighbour);
        //println!();
        for mut block in neighbour {
            block.cout = current.cout + map[block.y][block.x];
            if block.cout > visited[block.y][block.x][block.dir as usize][block.consecutive as usize] { continue; }
            block.heuristic = heuristic((block.y, block.x), exit_pos);
            visited[block.y][block.x][block.dir as usize][block.consecutive as usize] = block.cout;
            //pred[block.y][block.x][block.dir as usize] = (current.y, current.x, current.dir as usize);
            let prio = block.heuristic + block.cout;
            pq.push(block, -prio);
        }
    }
    
    println!("TIME {} : ", start.elapsed().as_micros());
    println!("{}", finisher.cout);
    //tracert(&map, &pred, (finisher.y, finisher.x, finisher.dir as usize), w, h) ;

    
}

fn tracert (map : &Vec<Vec<i32>>, pred : &Vec<Vec<Vec<(usize, usize, usize)>>>, exit_pos : (usize, usize, usize), w : usize, h: usize) {
    let mut a = vec![vec![5;w];h];
    let mut curr = pred[exit_pos.0][exit_pos.1][exit_pos.2];
    loop {
        a[curr.0][curr.1] = curr.2;
        curr = pred[curr.0][curr.1][curr.2];
        if curr.0 == 0 && curr.1 == 0 {
            break;
        }
    }
    for (i,v) in a.iter().enumerate() {
        for (k, c) in v.iter().enumerate() {
            if *c == 5 {
                print!("{}", map[i][k]);
            }
            else if *c == 0 {
                print!("{}", "^".red());
            }
            else if *c == 1 {
                print!("{}", "v".red());
            }
            else if *c == 2 {
                print!("{}","<".red());
            }
            else if *c == 3 {
                print!("{}", ">".red());
            }
        }
        println!();
    }
}

fn main() {
    p1();
}
