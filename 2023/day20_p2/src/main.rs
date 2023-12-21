use std::{fs, collections::{HashMap, VecDeque, BTreeMap}};
use colored::Colorize;

#[derive(Debug, Clone)]
struct Broadcast {
    dest_mod : Vec<usize>,
}
trait Module {
    fn get_type(&self) -> u8 ;
    fn get_pulses(&mut self, source : Pulse) -> Option<VecDeque<Pulse>>;
    fn init_memory(&mut self, from : usize);
    fn get_dest(&mut self) -> Vec<usize>;
}

#[derive(Debug, Clone, Copy)]
struct Pulse {
    pusle_type : bool,
    dest : usize,
    from : usize,
}
#[derive(Debug)]
struct FlipFlop {
    mtype : u8,
    name : String,
    index_name : usize,
    state : bool,
    dest_mod : Vec<usize>,
}
impl Module for FlipFlop {
    fn get_pulses(&mut self, source : Pulse) -> Option<VecDeque<Pulse>>  {
        //println!("{}", self.name.red());
        if !source.pusle_type {
            
            self.state ^= true;
            let mut send = VecDeque::with_capacity(self.dest_mod.capacity());
            for d in &self.dest_mod {
                send.push_back(Pulse {dest : *d, pusle_type : self.state, from: self.index_name});
            }
            return Some(send);
        }
        None
    }
    fn get_type(&self) -> u8 {
        return self.mtype;
    }
    fn init_memory(&mut self, from : usize) {
        
    }
    fn get_dest(&mut self) -> Vec<usize> {
        self.dest_mod.clone()
    }
}
#[derive(Debug)]
pub struct Conjonction {
    mtype : u8,
    name : String,
    index_name : usize,
    pub mod_type : char,
    dest_mod : Vec<usize>,
    memory : BTreeMap<usize, bool>
}
impl Module for Conjonction {
    fn get_pulses(&mut self, source : Pulse) -> Option<VecDeque<Pulse>> {
        //println!("{}", self.name.blue());
        match self.memory.get_mut(&source.from) {
            Some(x) => {
                *x = source.pusle_type;
            },
            None => {
                self.memory.insert(source.from, source.pusle_type);
            }
        }
        let mut r =  true;
        for v in self.memory.values() {
            if *v == false { r = false; break;}
        }

        let mut send = VecDeque::with_capacity(self.dest_mod.capacity());
        for d in &self.dest_mod {
            send.push_back(Pulse {dest : *d, pusle_type : !r, from: self.index_name});
        }
        return Some(send);
    }
    fn get_type(&self) -> u8 {
        return self.mtype;
    }
    fn init_memory(&mut self, from : usize) {
        self.memory.insert(from, false);
    }
    fn get_dest(&mut self) -> Vec<usize> {
        self.dest_mod.clone()
    }
}
fn simulation( modules : &mut Vec<Box<dyn Module>>, n : usize, broadcast : Broadcast) -> u64 {
    let mut pulse_queue_org = VecDeque::new();
    for a in broadcast.dest_mod {
        pulse_queue_org.push_back(Pulse{ dest : a, pusle_type : false, from : 0});
    }
    let mut nb_hight = 0;
    //let mut nb_low = 1;
    let mut nb_low = n as u64;

    
    //for i in 0..n {
    let mut i = 0;
    loop {
        let mut k = 0;
        let mut pulse_queue = pulse_queue_org.clone();
        while !pulse_queue.is_empty() {
            let current_pulse = pulse_queue.pop_front().unwrap();
            //println!("current{:?}", current_pulse);
            /*match current_pulse.pusle_type {
                true  => nb_hight += 1,
                false => nb_low   += 1,
            }*/
            if current_pulse.dest == usize::MAX {
                if  current_pulse.pusle_type == false {
                    //println!("BREAK {}", i);
                    break;
                }
                continue;
            }
            if let Some(mut vp) = modules[current_pulse.dest].get_pulses(current_pulse) {
                //println!("vp from {}", current_pulse.from);
                pulse_queue.append(&mut vp);
            }
            //println!();

            //println!("break ; {:?}", pulse_queue);
            //println!();
            k+=1;
        }
        println!("{k}");
        //break;
        break;
        i+=1;
    }
    
    println!("{} {}", nb_hight, nb_low);
    //println!("{} {}", nb_hight, nb_low+n as u64);
    nb_hight * (nb_low)
}

fn p2() {
    let (mut modules, b, modules_name) = init();
    let n = 1000;
    let res = simulation(&mut modules, n, b);
    //dfs(&mut modules, b );
    println!("{}", res);
    
    let r = ["nd", "ds", "hf", "sb"];
    let mut e = Vec::new();
    for node1 in r {
        let (mut modules, b, modules_names) = init();
        let node = modules_name.get(node1).unwrap();
        let mut count = 1;
        let count = found_node(&mut modules, b.clone(), *node);
        println!("{}", count);
        e.push(count);
    }
    println!("{}", findlcm(e.clone(), e.len()));
}
fn found_node( modules : &mut Vec<Box<dyn Module>>, broadcast : Broadcast, node : usize) -> u64 {
    let mut pulse_queue_org = VecDeque::new();
    for a in broadcast.dest_mod {
        pulse_queue_org.push_back(Pulse{ dest : a, pusle_type : false, from : 0});
    }   
    
    let mut i = 1;
    loop {
        let mut pulse_queue = pulse_queue_org.clone();
        while !pulse_queue.is_empty() {
            let current_pulse = pulse_queue.pop_front().unwrap();
            
            if current_pulse.dest == node && !current_pulse.pusle_type {
                return i;
            }
            if current_pulse.dest == usize::MAX {
                if  current_pulse.pusle_type == false {
                    //println!("BREAK {}", i);
                    break;
                }
                continue;
            }
            if let Some(mut vp) = modules[current_pulse.dest].get_pulses(current_pulse) {
                //println!("vp from {}", current_pulse.from);
                pulse_queue.append(&mut vp);
            }
            
        }
        i+=1;
    }
    i
}

fn init() -> (Vec<Box<dyn Module>>, Broadcast, HashMap<String, usize>) {
    //let file = fs::read_to_string("example").unwrap();
    //let file = fs::read_to_string("example2").unwrap();
    let file = fs::read_to_string("input").unwrap();
    let mut modules_names = HashMap::new();
    let mut modules: Vec<Box<dyn Module>> = Vec::new();
    let mut b = Broadcast { dest_mod : Vec::new() };

    let mut i = 0;
    let mut broadcast_index = "";
    for line in file.lines() {
        let s : Vec<&str> = line.split(" -> ").collect();
        let mod_type = s[0].chars().nth(0).unwrap();
        if mod_type != 'b' {
            let name = &s[0][1..];
            modules_names.insert(name.to_string(), i);
            i+=1;
        }
        if line.starts_with("broadcast") {
            broadcast_index = line;
        }
    }
    
    let r :Vec<&str> = broadcast_index.split(" -> ").collect();
    for x in  r[1].split(',') {
        b.dest_mod.push(*(modules_names.get(x.trim()).unwrap()));
    }
    for line in file.lines() {
        
        let s : Vec<&str> = line.split(" -> ").collect();
        let mod_type = s[0].chars().nth(0).unwrap();
        //println!("{}", line);
        if mod_type == '&' {
            let name = &s[0][1..];
            let mut dest_mod = Vec::new();
            for next in s[1].split(',') {
                let index_name = match modules_names.get(next.trim()){
                    Some(x) => *x,
                    None => usize::MAX,
                };
                dest_mod.push(index_name);
            }
            let index_name = *(modules_names.get(name).unwrap());
            let conj = Conjonction {
                mtype : 2,
                name : name.to_string(),
                mod_type,
                index_name : index_name,
                dest_mod,
                memory : BTreeMap::new(),
            };
            modules.push(Box::new(conj));
        } 
        if mod_type == '%' {
            let name = &s[0][1..];
            let mut dest_mod = Vec::new();
            for next in s[1].split(',') {
                let index_name = match modules_names.get(next.trim()) {
                    Some(x) => *x,
                    None => usize::MAX,
                };
                dest_mod.push(index_name);
            }
            let index_name = *(modules_names.get(name).unwrap());
            let conj = FlipFlop {
                mtype : 1,
                name : name.to_string(),
                index_name, 
                state : false,
                dest_mod,
            };
            modules.push(Box::new(conj));
        }
    }
    for line in file.lines() { // Init
        let s : Vec<&str> = line.split(" -> ").collect();
        let mod_type = s[0].chars().nth(0).unwrap();
        if mod_type != 'b' {
            let name = &s[0][1..];
            let name_index = *(modules_names.get(&name.to_string()).unwrap());
            for next in s[1].split(',') {
                match modules_names.get(next.trim()){
                    Some(x) => {
                        let r = modules.get_mut(*x).unwrap();
                        if r.get_type() == 2 {
                            r.init_memory(name_index);
                        }
                    },
                    None => {},
                };
            }
        }
    }
    (modules, b, modules_names)
}

fn dfs (modules : &mut Vec<Box<dyn Module>>, broadcast : Broadcast) {
    let mut pulse_queue_org = VecDeque::new();
    let mut already_visited = vec![false;modules.len()];
    for a in broadcast.dest_mod {
        pulse_queue_org.push_back(Pulse{ dest : a, pusle_type : false, from : 0});
        rec(modules, &mut already_visited, a, 0);
    }
}
fn rec(modules : &mut Vec<Box<dyn Module>>, already_visited : &mut Vec<bool>, curr : usize, count: usize) {
    for i in 0..count {
        print!(" ");
    }
    println!("{} {}", curr, modules[curr].get_type() );
    already_visited[curr] = true;
    for a in modules[curr].get_dest() {
        if a != usize::MAX && !already_visited[a] {
            rec(modules, already_visited, a, count+1);
        }
    }
}

fn main() {
    p2();
}



fn gcd(a: u64, b : u64) -> u64
{
    if (b == 0){
        return a;
    }
    return gcd(b, a % b);
}
 
// Returns LCM of array elements
fn findlcm(arr : Vec<u64>, n : usize) -> u64
{
    // Initialize result
    let mut ans = arr[0];
 
    // ans contains LCM of arr[0], ..arr[i]
    // after i'th iteration,
    for i in 1..n {
        ans = (((arr[i] * ans)) /
                (gcd(arr[i], ans)));
    }
    return ans;
}