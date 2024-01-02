use std::{fs, collections::HashMap};
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use rustworkx_core::Result;

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut mod_name = HashMap::new();
    let mut graph : UnGraph<(), ()> = UnGraph::new_undirected();
    for line in file.lines() {
        let s : Vec<&str>  = line.split(':').collect();
        let node = s[0];
        let r :Vec<&str> = s[1].trim().split_ascii_whitespace().collect();
        if let None =  mod_name.get(node) {
            let n = graph.add_node(());
            mod_name.insert(node, n);
        };
        for a in r {
            if let None =  mod_name.get(a) {
                let n = graph.add_node(());
                mod_name.insert(a, n);
            }
            let s = mod_name.get(node).unwrap();
            let d = mod_name.get(a).unwrap();
            graph.extend_with_edges(&[(*s, *d)]);
        }
    }
    let min_cut_res: Result<Option<(usize, Vec<_>)>> =
    stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (min_cut, partition) = min_cut_res.unwrap().unwrap();
   
    println!("{}", min_cut);
    println!("{:?} {}", (graph.node_count()- partition.len()), partition.len());
    
}


fn main() {
    p1();
}