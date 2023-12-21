use std::{fs, collections::HashMap};

struct Rules {
    categorie : char,
    eq_type : char,
    threshold : u64,
    next : String,
}

struct Workflow {
    rules : Vec<Rules>, 
    last : String
}

fn convert_cate_to_nb (c : char) -> usize {
    match c {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's'=> 3,
        _ => 0
    }
}
fn process_workflow(hash_map : &mut HashMap<String, Workflow>, line : &str) {
    let mut rules_finale = Vec::new();
    let mut last = String::new();
    let s : Vec<&str> = line.split('{').collect();
    let workflow_name = s[0].to_string();
    let rules = s[1][..s[1].len()-1].split(',');
    for rule in rules {
        if !rule.contains(':') {
            last = rule.to_string();
            continue;
        }
        let categorie = rule.chars().nth(0).unwrap();
        let eq_type = rule.chars().nth(1).unwrap();
        let mut y = rule[2..].split(':');
        let threshold = y.next().unwrap().parse().unwrap();
        let next = y.next().unwrap().to_string();
        let r = Rules {categorie , eq_type, threshold , next };
        rules_finale.push(r);
    }
    hash_map.insert(workflow_name, Workflow { rules : rules_finale, last});
}

fn get_possi (line : [(u64, u64);4]) -> u64 {
    let mut res = 1;
    for part in line {        
        res *= part.1+1 - part.0;
    }
    res
}
fn rec(workflow : &HashMap<String, Workflow>, mut range : [(u64, u64);4], wf_name: String) -> u64 {
    if wf_name == "A" {
        return get_possi(range);
    }
    else if wf_name == "R" {
        return 0;
    }
    let mut nb_accepted = 0;
    let wf = workflow.get(&wf_name).unwrap();
    for rule in &wf.rules {
        let d = convert_cate_to_nb(rule.categorie);
        match rule.eq_type {
            '<' => {
                if  range[d].0 >= rule.threshold {
                    continue;
                }
                else if range[d].0 < rule.threshold && rule.threshold <= range[d].1 {
                    let mut split = range;
                    split[d].1 = rule.threshold-1;
                    range[d].0 = rule.threshold;
                    nb_accepted += rec(workflow, split, rule.next.clone());

                }
                else if range[d].1 < rule.threshold {
                    nb_accepted += rec(workflow, range, rule.next.clone());
                    break;
                }
            },
            '>' => {
                if  range[d].1 <= rule.threshold {
                    continue;
                }
                else if range[d].1 > rule.threshold && range[d].0 <= rule.threshold {
                    let mut split = range;
                    split[d].0 = rule.threshold+1;
                    range[d].1 = rule.threshold;
                    nb_accepted += rec(workflow, split, rule.next.clone());

                }
                else if range[d].0 > rule.threshold {
                    nb_accepted += rec(workflow, range, rule.next.clone());
                    break;
                }
            },_=>{}
        }
    }
    nb_accepted += rec(workflow, range, wf.last.clone());
    nb_accepted
}
fn p2() {
    let mut res = 0;
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut workflow : HashMap<String, Workflow> = HashMap::new();
    
    for line in file.lines() {
        if line.is_empty() {
            break;
        }
        process_workflow(&mut workflow, line);
    }
    let range = [(1, 4000), (1,4000), (1,4000), (1,4000)];
    res = rec(&workflow, range, "in".to_string());
    println!("{}", res);
}


fn main() {
    p2();
}
