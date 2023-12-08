use core::time;
use std::{fs, collections::{BTreeMap}};
#[derive(Debug)]
struct Node {
    left : String,
    right : String,
}
fn p1(){
    let mut map :BTreeMap<String, Node> = BTreeMap::new();
    let file = fs::read_to_string("input").unwrap();
    let mut iter = file.lines();
    let direction = iter.next().unwrap();
    iter.next();
    for line in iter {
        let split : Vec<&str> = line.split('=').collect();
        let source = split[0].trim();
        let mut dest = split[1][2..split[1].len()-1].split(',');
        map.insert(source.to_string(), 
            Node { 
                left : dest.next().unwrap().to_string(),
                right : dest.next().unwrap().trim().to_string()
            }
        );
    }
    let mut nb_step = 0;
    let mut actual_node = "AAA".to_string();
    while actual_node != "ZZZ" {
        for dir in direction.chars() {
            let node = map.get(&actual_node).unwrap();
            actual_node = match dir {
                'L' => {
                    node.left.clone()
                },
                'R' => {
                    node.right.clone()
                },
                _=> { node.right.clone() }
            };
            nb_step+=1;
            if actual_node == "ZZZ" {
                break;
            }
        }
    }
    println!("{}", nb_step);
    
}
fn p2(){
    let mut map :BTreeMap<String, Node> = BTreeMap::new();
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut iter = file.lines();
    let direction = iter.next().unwrap();
    iter.next();
    let mut ends_A : Vec<String> = Vec::new();
    for line in iter {
        let split : Vec<&str> = line.split('=').collect();
        let source = split[0].trim();
        let mut dest = split[1][2..split[1].len()-1].split(',');
        if source.ends_with('A') {
            ends_A.push(source.to_string());
        }
        map.insert(source.to_string(), 
            Node { 
                left : dest.next().unwrap().to_string(),
                right : dest.next().unwrap().trim().to_string()
            }
        );
    }
    //FIND THE Z NODE
    let mut time_to_first_z = Vec::new();
    let mut first_Z = Vec::new();
    for a in &ends_A {
        let mut nb_step = 0;
        let mut actual_node = (*a).clone();
        while !actual_node.ends_with('Z') {
            for dir in direction.chars() {
                let node = map.get(&actual_node).unwrap();
                actual_node = match dir {
                    'L' => {
                        node.left.clone()
                    },
                    'R' => {
                        node.right.clone()
                    },
                    _=> { node.right.clone() }
                };
                nb_step+=1;
                if actual_node.ends_with('Z'){
                    break;
                }
            }
        }
        time_to_first_z.push(nb_step);
        first_Z.push(actual_node);
    }
    println!("{:?}", time_to_first_z);
    println!("{:?}", first_Z);
    let mut cycle_time = Vec::new();
    for (i, a) in (&first_Z).iter().enumerate() {
        let mut nb_step = 0;
        let mut actual_node = (*a).clone();
        println!(" {} {} {}", direction.len(), time_to_first_z[i], time_to_first_z[i]%direction.len());
        
        loop {
            for dir in direction[time_to_first_z[i]%direction.len()..].chars() {
                let node = map.get(&actual_node).unwrap();
                actual_node = match dir {
                    'L' => {
                        node.left.clone()
                    },
                    'R' => {
                        node.right.clone()
                    },
                    _=> { node.right.clone() }
                };
                nb_step+=1;
                if actual_node == *a{
                    break;
                }
            }
            if actual_node == *a {
                break;
            }
        }
        cycle_time.push(nb_step);
    }
    let res = findlcm(&cycle_time, cycle_time.len());
    println!("{:?}", res);
    
}


fn gcd( a : usize, b : usize) -> usize {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
 
// Returns LCM of array elements
fn findlcm(arr : &[usize], n : usize) -> usize {
    // Initialize result
    let mut ans = arr[0];
 
    // ans contains LCM of arr[0], ..arr[i]
    // after i'th iteration,
    for  i in 0..n {
        ans = ((arr[i] * ans)) / (gcd(arr[i], ans));
    }
    return ans;
}


fn all_node_end_by_Z(nodes : &Vec<String>) -> bool {
    for n in nodes {
        if !n.ends_with('Z') {
            return false;
        }
    }
    true
}

fn main() {
    //p1();
    p2();
}
