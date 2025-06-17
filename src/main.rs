use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

mod temporal_graphs;
use ontime::parser::tg_parser;


fn main() -> io::Result<()> {
    // Path to the example file
    let path = Path::new("examples/att.tg");
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    // Parse the file
    let parser = tg_parser::AttrParser::new();
    match parser.parse(&input) {
        Ok(graph) => {
            println!("{:#?}", graph);
        }
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
        }
    }

    Ok(())
}
