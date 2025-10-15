use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::time::Instant;

use clap::Parser;
use ontime::game::reachable_at;
use ontime::parser::tg_parser::{NIDListParser, TemporalGraphParser};

/// A solver for punctual reachability games on temporal graphs
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the temporal graph input file (use '-' for stdin)
    input_file: Option<String>,
    
    /// Target set of nodes (comma-separated node IDs)
    #[arg(long, default_value = "v0")]
    target_set: String,
    
    /// Time to reach the target set (will be overridden by .meta file if present)
    #[arg(long, default_value = "10")]
    time_to_reach: usize,
    
    /// Output only timing information (compatible with GGG benchmark)
    #[arg(long)]
    time_only: bool,
    
    /// Output solver name and exit
    #[arg(long)]
    solver_name: bool,
    
    /// Output in CSV format
    #[arg(long)]
    csv: bool,
}

fn read_time_bound_from_meta(file_path: &str) -> Option<usize> {
    // Convert .tg file to .meta file path
    let meta_path = file_path.replace(".tg", ".meta");
    
    if let Ok(mut file) = File::open(&meta_path) {
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() {
            for line in content.lines() {
                if let Some(time_bound_str) = line.strip_prefix("time_bound: ") {
                    if let Ok(time_bound) = time_bound_str.trim().parse::<usize>() {
                        return Some(time_bound);
                    }
                }
            }
        }
    }
    None
}

fn extract_time_bound_from_tg_content(content: &str) -> Option<usize> {
    // Look for time_bound in comment lines
    for line in content.lines() {
        if let Some(time_bound_str) = line.strip_prefix("// time_bound: ") {
            if let Ok(time_bound) = time_bound_str.trim().parse::<usize>() {
                return Some(time_bound);
            }
        }
    }
    None
}

fn extract_targets_from_tg_content(content: &str) -> Option<String> {
    // Look for targets in comment lines
    for line in content.lines() {
        if let Some(targets_str) = line.strip_prefix("// targets: ") {
            return Some(targets_str.trim().to_string());
        }
    }
    None
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    // Handle solver name request
    if args.solver_name {
        println!("Ontime Punctual Reachability Solver");
        return Ok(());
    }

    let start_time = Instant::now();
    
    // Read input (from file or stdin)
    let input = if let Some(file_path) = &args.input_file {
        if file_path == "-" {
            // Read from stdin
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            input
        } else {
            // Read from file
            let path = Path::new(file_path);
            let mut file = File::open(path)?;
            let mut input = String::new();
            file.read_to_string(&mut input)?;
            input
        }
    } else {
        // Default to stdin if no file specified
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        input
    };

    // Parse the file
    let parser = TemporalGraphParser::new();
    let graph = parser.parse(&input).expect("Parse error");

    // Determine time bound - priority order:
    // 1. From TG file content (works with stdin)
    // 2. From .meta file (only when file path available)
    // 3. Command line argument (fallback)
    let k: usize = extract_time_bound_from_tg_content(&input)
        .or_else(|| {
            if let Some(file_path) = &args.input_file {
                if file_path != "-" {
                    read_time_bound_from_meta(file_path)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(args.time_to_reach);

    // Determine target set - priority order:
    // 1. From TG file content (works with stdin)
    // 2. Command line argument (fallback)
    let target_set = extract_targets_from_tg_content(&input)
        .unwrap_or(args.target_set.clone());

    // parse target
    let parser = NIDListParser::new();
    let v = parser.parse(&target_set).expect("Failed to read target");
    let target_ids: std::collections::HashSet<_> = v.iter().cloned().collect();

    // w is the winning set at time k
    let target_at_k: Vec<bool> = graph.nodes_selected_from_ids(&target_ids);
    
    // compute the reachable set at time 0
    let wins_at = reachable_at(&graph, k, true, &target_at_k);
    
    let solve_time = start_time.elapsed();
    
    // Output based on requested format
    if args.time_only {
        // Output only timing (for GGG benchmark compatibility)
        println!("{:.6}", solve_time.as_secs_f64());
    } else if args.csv {
        // CSV format compatible with GGG
        let filename = args.input_file.as_deref().unwrap_or("stdin");
        println!("Ontime Punctual Reachability Solver,{},solved,{:.6}",
                 filename, solve_time.as_secs_f64());
    } else {
        // Standard output
        println!("W_{} = {:?}", k, graph.ids_from_nodes_vec(&target_at_k));
        println!("W_0 = {:?}", graph.ids_from_nodes_vec(&wins_at));
    }

    Ok(())
}
