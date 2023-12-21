use std::{fs, collections::{HashMap, VecDeque, BTreeMap}};
use colored::Colorize;

#[derive(Debug)]
struct Broadcast {
    dest_mod : Vec<usize>,
}
trait Module {
    fn get_type(&self) -> u8 ;
    fn get_pulses(&mut self, source : Pulse) -> Option<VecDeque<Pulse>>;
    fn init_memory(&mut self, from : usize);
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
        let mut pulse_queue = pulse_queue_org.clone();
        while !pulse_queue.is_empty() {
            let current_pulse = pulse_queue.pop_front().unwrap();
            //println!("current{:?}", current_pulse);
            match current_pulse.pusle_type {
                true  => nb_hight += 1,
                false => nb_low   += 1,
            }
            if current_pulse.dest == usize::MAX {
                if  current_pulse.pusle_type == false {
                    println!("BREAK {}", i);
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
        }
        //break;
        i+=1;
    }
    
    println!("{} {}", nb_hight, nb_low);
    //println!("{} {}", nb_hight, nb_low+n as u64);
    nb_hight * (nb_low)
}

fn p1() {
    //let file = fs::read_to_string("example").unwrap();
    //let file = fs::read_to_string("example2").unwrap();
    let file = fs::read_to_string("input").unwrap();
    let mut modules_names = HashMap::new();
    let mut modules: Vec<Box<dyn Module>> = Vec::new();
    let mut i = 0;
    let mut broadcast_index = "";
    for line in file.lines() {
        let s : Vec<&str> = line.split(" -> ").collect();
        let mod_type = s[0].chars().nth(0).unwrap();
        if mod_type != 'b' {
            let name = &s[0][1..];
            modules_names.insert(name, i);
            i+=1;
        }
        if line.starts_with("broadcast") {
            broadcast_index = line;
        }
    }
    
    let r :Vec<&str> = broadcast_index.split(" -> ").collect();
    let mut b = Broadcast { dest_mod : Vec::new() };
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
            let name_index = *(modules_names.get(name).unwrap());
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
    let n = 1000;
    let res = simulation(&mut modules, n, b);
    println!("{}", res);
}

fn main() {
    p1();
}
