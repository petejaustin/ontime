use std::collections::HashSet;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use ontime::parser::tg_parser::{NIDListParser, TemporalGraphParser};
use ontime::temporal_graphs;

fn main() -> io::Result<()> {
    // Path to the example file
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "Usage: {} <input_file> <target_set> <time_to_reach>",
            args[0]
        );
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    // Parse the file
    let parser = TemporalGraphParser::new();
    let graph = parser.parse(&input).expect("Parse error");
    println!("{:#?}", graph);

    // println!("Target: {:?}", target_nodes);
    // println!("{:?}", graph.nodes_selected_from_ids(&target_nodes));

    // parse target
    let parser = NIDListParser::new();
    // let mut target_nodes: HashSet<String> = HashSet::new();
    // target_nodes.insert("s3".to_string());
    // target_nodes.insert("s0".to_string());
    let v = parser.parse(&args[2]).expect("Failed to read target");
    let target_ids: std::collections::HashSet<_> = v.iter().cloned().collect();
    println!("{:#?}", target_ids);

    println!("Target: {:?}", target_ids);

    // time to reach
    let k: usize = args[3].parse::<usize>().expect("Failed to parse usize");

    // W is the winning set at time k
    let mut W: Vec<bool> = graph.nodes_selected_from_ids(&target_ids);
    let owns: Vec<bool> = graph.node_ownership();
    println!("{:?}", owns);
    for i in (0..k).rev() {
        println!("{} {:?}", i, W);

        let mut W_new: Vec<bool> = vec![false; graph.node_count];
         for node in graph.nodes(){
             match owns[node] {
                true => W_new[node] = graph.successors_at(node, i).any(|s| W[s]),
                false => W_new[node] = graph.successors_at(node, i).all(|s| W[s]),
            }
            W = W_new.clone();
        }
    }

    Ok(())
}
