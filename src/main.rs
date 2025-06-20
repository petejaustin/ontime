use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use ontime::temporal_graphs;
use ontime::parser::tg_parser;


fn main() -> io::Result<()> {
    // Path to the example file
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new("examples/game1.1.tg")
    };
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut target_nodes: HashSet<String> = HashSet::new();
    target_nodes.insert("s3".to_string());


    // Parse the file
    let parser = tg_parser::TemporalGraphParser::new();
    match parser.parse(&input) {
        Ok(graph) => {
            println!("{:#?}", graph);

            println!("Target: {:?}", target_nodes);
            println!("{:?}", graph.nodes_selected_from_ids(&target_nodes));
        }
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
        }
    }

    Ok(())
}
