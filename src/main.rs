use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use clap::Parser;
use ontime::game::reachable_at;
use ontime::parser::tg_parser::{NIDListParser, TemporalGraphParser};

/// A solver for punctual reachability games on temporal graphs
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the temporal graph input file
    input_file: String,
    
    /// Target set of nodes (comma-separated node IDs)
    target_set: String,
    
    /// Time to reach the target set
    time_to_reach: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let path = Path::new(&args.input_file);
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    // Parse the file
    let parser = TemporalGraphParser::new();
    let graph = parser.parse(&input).expect("Parse error");
    println!("{:#?}", &graph);

    // parse target
    let parser = NIDListParser::new();
    let v = parser.parse(&args.target_set).expect("Failed to read target");
    let target_ids: std::collections::HashSet<_> = v.iter().cloned().collect();
    println!("\n\ntarget {:#?}", target_ids);

    // time to reach
    let k: usize = args.time_to_reach;

    // w is the winning set at time k
    let target_at_k: Vec<bool> = graph.nodes_selected_from_ids(&target_ids);
    println!("W_{} = {:?}", k, graph.ids_from_nodes_vec(&target_at_k));

    // compute the reachable set at time 0
    let wins_at = reachable_at(&graph, k, true, &target_at_k);

    // output
    println!("W_0 = {:?}", graph.ids_from_nodes_vec(&wins_at));

    Ok(())
}
