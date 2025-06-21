use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use ontime::parser::tg_parser::{NIDListParser, TemporalGraphParser};

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

    // parse target
    let parser = NIDListParser::new();
    let v = parser.parse(&args[2]).expect("Failed to read target");
    let target_ids: std::collections::HashSet<_> = v.iter().cloned().collect();
    //println!("{:#?}", target_ids);

    // time to reach
    let k: usize = args[3].parse::<usize>().expect("Failed to parse usize");

    // node ownershopt. true --> pLayer one, false --> player two node.
    let owns: Vec<bool> = graph.node_ownership();

    // w is the winning set at time k
    let mut wins_at: Vec<bool> = graph.nodes_selected_from_ids(&target_ids);
    println!("W_{} = {:?}", k, graph.ids_from_nodes_vec(&wins_at));

    // compute wins_at one at a time from k-1 down to 0
    for i in (0..k).rev() {
        // new empty vector
        let mut w: Vec<bool> = vec![false; graph.node_count];

        // 1-step attractor attime i
        for node in graph.nodes() {
            match owns[node] {
                true => w[node] = graph.successors_at(node, i).any(|s| wins_at[s]),
                false => w[node] = graph.successors_at(node, i).all(|s| wins_at[s]),
            }
        }
        wins_at = w.clone();
        //println!("W_{} = {:?}", i, graph.ids_from_nodes_vec(&wins_at));
    }

    // output
    println!("W_0 = {:?}", graph.ids_from_nodes_vec(&wins_at));

    Ok(())
}
