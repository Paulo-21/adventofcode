use std::{fs, collections::HashMap};

struct Rules {
    categorie : char,
    eq_type : char,
    threshold : i32,
    next : String,
}

struct Workflow {
    rules : Vec<Rules>, 
    last : String
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
fn match_rule(line : &str, rule : &Rules) -> bool {
    let s = line[1..line.len()-1].split(',');
    for part in s {
        let c = part.chars().nth(0).unwrap();
        if rule.categorie == c {
            let nb : i32 = part[2..].parse().unwrap();
            match rule.eq_type {
                '<' => return nb < rule.threshold,
                '>' => return nb > rule.threshold,
                _ => return false,
            }
        }
    }
    false
}
fn is_accepted(worflow : &HashMap<String, Workflow>, line : &str) -> bool {
    let mut current_workflow = "in";
    loop {
        let mut no_found = true;
        let w = worflow.get(current_workflow).unwrap();
        for rule in &w.rules {
            if match_rule(line, rule)  {
                no_found = false;
                if rule.next == "A" {
                    return true;
                }
                else if rule.next == "R" {
                    return false;
                }
                current_workflow = &rule.next;
                break;
            }
        }
        if no_found {
            current_workflow = &w.last;
        }
        if current_workflow == "A" {
            return true;
        }
        else if current_workflow == "R" {
            return false;
        }
    }
}
fn get_line_v (line : &str) -> i32 {
    let s = line[1..line.len()-1].split(',');
    let mut res = 0;
    for part in s {        
        res += part[2..].parse::<i32>().unwrap();   
    }
    res
}

fn p1() {
    let mut res = 0;
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut workflow : HashMap<String, Workflow> = HashMap::new();

    let mut workflow_mode = true;
    for line in file.lines() {
        if line.is_empty() {
            workflow_mode = false;
            continue;
        }

        if workflow_mode {
            process_workflow(&mut workflow, line);
        }
        else {
            if is_accepted(&workflow, line) {
                println!("{}", line);
                res += get_line_v(line);
            }
        }

    }
    println!("{}", res);
}

fn main() {
    p1();
}
