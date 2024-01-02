use std::{fs, collections::{BTreeMap}, cmp::{max, min}, time::Instant};
use rustworkx_core::{petgraph::{self, graph::{self, Node}, data::Build, algo::all_simple_paths, visit::IntoNeighbors}, connectivity::{connected_components, number_connected_components}, shortest_path::dijkstra, dictmap::DictMap, traversal::dfs_edges};
use petgraph::prelude::*;
use hashbrown::HashSet;
use rustworkx_core::connectivity::longest_simple_path_multiple_targets;
use rustworkx_core::Result;

use fixedbitset::FixedBitSet;
const TRANSFORM : [(i32, i32); 4] = [(0,1),(0,-1),(1,0),(-1,0)];

fn rec1(map : &[Vec<char>], dp : &mut HashSet<(i32, i32)>, mut current : (i32, i32), dir : usize, mut old : (i32, i32), mut nbstep : u32) -> u32 {
    if current.0 == (map.len()-1) as i32 && current.1 == (map[0].len()-2) as i32 {
        return nbstep;
    }
    let mut r = Vec::with_capacity(3);
    println!("{} {}", current.0, current.1);
    loop {
        nbstep+=1;
        if current.0 == (map.len()-1) as i32 && current.1 == (map[0].len()-2) as i32 {
            return nbstep;
        }
        for (k, t) in TRANSFORM.iter().enumerate() {
            let new = (current.0+t.0, current.1+t.1);
            if new.0 < 0 || new.0  >= map.len()as i32|| new.1 <0 || new.1 >= map[0].len()as i32 ||
            map[new.0 as usize][new.1 as usize] == '#' || (new.0 == old.0 && new.1 ==old.1)  {
                continue;
            }
            if map[new.0 as usize][new.1 as usize] == '>' && k != 0 {
                continue;
            } 
            if map[new.0 as usize][new.1 as usize] == 'v' && k != 2 {
                continue;
            } 
            r.push(new);
            dp.insert(new);
        }
        old = current;
        if r.len() > 1 {
            break;
        }
        
        current = r[0];
        r.clear();
    }
    let mut maxi = 0;
    for n in r {
        maxi = max(rec1(map, dp, n, dir, current, nbstep), maxi);
    }
    return maxi;
}

fn rec(map : &[Vec<char>], dp : &mut BTreeMap<((i32,i32),(i32,i32)), u32>, mut current : (i32, i32), mut old : (i32, i32), mut nbstep : u32, mut path : Vec<(i32,i32)>) -> (u32, Vec<(i32,i32)>) {
    let old_old = old;
    if current.0 == (map.len()-1) as i32 && current.1 == (map[0].len()-2) as i32 {
        return (nbstep, path);
    }
    match dp.get_mut(&(current, old_old)) {
        Some(x) => {
            //println!("{} {}", x, nbstep);
            if *x < nbstep {
                *x = nbstep;
            }
            else {
                //if !path.contains(&current) {
                return (nbstep, path);
                //}
            }
        },
        None => {
            dp.insert((current, old_old), nbstep);
        }
    }
    let mut r = Vec::with_capacity(3);
    //println!("{} {}", current.0, current.1);
    
    loop {
        nbstep+=1;
        if current.0 == (map.len()-1) as i32 && current.1 == (map[0].len()-2) as i32 {
            return (nbstep, path);
        }
        //println!("{:?}", current);
        for t in TRANSFORM {
            let new = (current.0+t.0, current.1+t.1);
            if new.0 < 0 || new.0  >= map.len()as i32|| new.1 <0 || new.1 >= map[0].len()as i32 ||
            map[new.0 as usize][new.1 as usize] == '#' || (new.0 == old.0 && new.1 ==old.1)  {
                continue;
            }
            if path.contains(&new) {
                continue;
            }
            r.push(new);
        }
        old = current;
        if r.len() > 1 {
            break;
        }
        if r.is_empty() {
            return (0, path);
        }
        
        current = r[0];
        r.clear();
        path.push(current);
    }
    let mut maxi = u32::MIN;
    let mut max_path = Vec::new();
    for n in r {
        let mut temp = path.clone();
        temp.push(n);
        //println!("FOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOORK");
        let (m, p) = rec(map, dp, n, current, nbstep, temp);
        if m > maxi {
            maxi = m;
            max_path = p;
        }
    }
    (maxi, max_path)
}


fn build_graph(map : &Vec<Vec<char>>, graph : &mut DiGraph<(), i32>, node_map : &mut BTreeMap<(i32,i32), NodeIndex>, from : NodeIndex, dest_indx : NodeIndex, mut current : (i32, i32), mut old : (i32, i32), mut nbstep : i32) {
    if current.0 == (map.len()-1) as i32 && current.1 == (map[0].len()-2) as i32 {
        graph.extend_with_edges(&[(from, dest_indx, nbstep)]);
        //println!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
        return;
    }
    let mut r = Vec::with_capacity(3);
    loop {
        nbstep+=1;
        if current.0 == (map.len()-1) as i32 && current.1 == (map[0].len()-2) as i32 {
            graph.extend_with_edges(&[(from, dest_indx, nbstep)]);
            //println!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
            return;
        }
        for t in TRANSFORM {
            let new = (current.0+t.0, current.1+t.1);
            if new.0 < 0 || new.0  >= map.len()as i32|| new.1 <0 || new.1 >= map[0].len()as i32 ||
            map[new.0 as usize][new.1 as usize] == '#' || (new.0 == old.0 && new.1 ==old.1)  {
                continue;
            }
            r.push(new);
        }
        old = current;
        if r.len() > 1 {
            break;
        }
        if r.is_empty(){
            //println!("PPPPPPPPPPPPPP");
        }
        current = r[0];
        r.clear();
    }
    let mut already = false;
    let new_node = match node_map.get(&current) {
        Some(x) => {
            already = true;
            graph.extend_with_edges(&[(*x, from, nbstep)]);
            graph.extend_with_edges(&[(from, *x, nbstep)]);
            *x
        },
        None => {
            let new_node = graph.add_node(());
            node_map.insert(current, new_node);
            //println!("{:?}", current);
            //println!("{:?} {:?}", new_node, from);
            graph.extend_with_edges(&[(from, new_node, nbstep)]);
            if from.index() != 0 {
                graph.extend_with_edges(&[(new_node, from, nbstep)]);
            }
            new_node
        }
    };
    if !already {
        for n in r {
            build_graph(map, graph, node_map,new_node, dest_indx, n, current, 0);
        }
    }
}

fn dfs(graph : &DiGraph<(), i32>, visited : &mut Vec<bool>/*FixedBitSet*/, current : NodeIndex , goal: NodeIndex, step : i32) -> i32{
    //println!("{:?}", current);
    if current == goal {
        //println!("FIND");
        return step;
    }
    let neighbors = graph.neighbors_directed(current, Outgoing);
    let mut m = 0;
    //visited.insert(current.index());
    visited[current.index()] = true;
    //println!();
    for n in neighbors {
        //println!(" {:?} {:?}", current, n);
        if visited[n.index()] {
            continue;
        }
        let edge = graph.find_edge(current, n).unwrap();
        m = max(m, dfs(graph, visited, n, goal, step+graph.edge_weight(edge).unwrap())) ;
    }
    //println!();
    visited[current.index()]= false;
    //visited.toggle(current.index());

    m
}

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut map : Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        map.push(line.chars().collect());
    }
    let mut graph = DiGraph::<(), i32>::new();
    let mut node_map = BTreeMap::<(i32,i32), NodeIndex>::new();
    
    let s = graph.add_node(());
    let d = graph.add_node(());
    build_graph(&map, &mut graph, &mut node_map, s,  d, (0,1), (0,1), 0);
    let mut to_set = HashSet::new();
    to_set.insert(d);

    println!("{:?}", graph);
    let mut v = vec![false;graph.node_count()];
    //let mut v = FixedBitSet::with_capacity(graph.node_count());
    let start = Instant::now();
    let res = dfs(&graph, &mut v, s, d, 0)-1;
    println!("{} us", start.elapsed().as_micros());
    //let mut a = number_connected_components(&graph);
    //let mut path = connected_components(&graph);

    /*let res : Result<DictMap<NodeIndex, usize>> = dijkstra(
        &graph, s, Some(d), |_| Ok(1), None
    ); 
    let r = all_simple_paths::<Vec<_>, _>(&graph, s, d, 0, None)
  .collect::<Vec<_>>();*/
    //let mut path = res.unwrap();
    //let mut path = longest_simple_path_multiple_targets(&graph, s, &to_set).unwrap();
    //print_map(&map, &path);
    /*println!("FINISH");
    let mut m = 0;
    for pa in r {
        let mut res = 0;
        for n in &pa {
            print!("{} ", n.index());
        }
        let mut path = pa.clone();
        let mut curr = path.pop().unwrap();
        while !path.is_empty() {
            let next = path.pop().unwrap();
            let edge = graph.find_edge(next, curr).unwrap();
            res += graph.edge_weight(edge).unwrap();
            curr = next;
        }
        println!();
        println!("{}", res);
        println!();
        println!();
        m = max(m, res);
    }*/
    /*let mut res = 0;
    let mut curr = path.pop().unwrap();
    while !path.is_empty() {
        let next = path.pop().unwrap();
        let edge = graph.find_edge(next, curr).unwrap();
        res += graph.edge_weight(edge).unwrap();
        curr = next;
    }*/
    //println!("{}", res-1);
    //println!("{:?}", path);
    /*println!("{:?}", path[0]);
    println!("{:?}", a);*/
    println!("{}", res);
}

fn print_map(map : &Vec<Vec<char>>, dp : &Vec<(i32, i32)>) {
    let mut u = 0;
    for (i, v) in map.iter().enumerate() {
        for (k, c) in v.iter().enumerate() {
            if dp.contains(&(i as i32, k as i32)) {
                print!("O");
                u += 1;
            }
            else {
                print!("{}", *c);
            }
        }
        println!();
    }

    println!("{u}");
}

fn main() {
    p1();
}
