use std::collections::HashMap;

use lalrpop_util::lalrpop_mod;


use crate::{temporal_graphs::{Edge, Node, TemporalGraph}};
use crate::formulae::Formula;


#[derive(Debug, Clone)]
pub enum NodeAttr {
    Label(String),
    Owner(bool),
}


#[derive(Debug)]
pub enum ParsedLine {
    Node(String, Vec<NodeAttr>),
    Edge(String, String, Option<Formula>),
    Empty,
}

lalrpop_mod!(pub tg_parser, "/parser/tg_parser.rs"); // LALRPOP parser module
lalrpop_mod!(pub formula, "/parser/formula.rs"); // LALRPOP parser module


pub fn temporal_graph_from_lines(lines: Vec<ParsedLine>) -> TemporalGraph {
        // first collect all nodes and edges
        let mut node_lines = Vec::new();
        let mut edge_lines = Vec::new();
        for item in lines {
            match item {
                ParsedLine::Node(_,_) => node_lines.push(item),
                ParsedLine::Edge(_,_,_) => edge_lines.push(item),
                ParsedLine::Empty => {},
            }
        }

        // Map string node IDs to indices
        let mut id_map = HashMap::new();
        let mut node_attrs: HashMap<Node, HashMap<String,NodeAttr>> = HashMap::new();
        let mut next_idx = 0;


        for item in &node_lines {
            if let ParsedLine::Node(id, attrs) = item {
                let idx = *id_map.entry(id.clone()).or_insert_with(|| {
                    let i = next_idx;
                    next_idx += 1;
                    i
                });

                let mut attr_map = HashMap::<String, NodeAttr>::new();

                for a in attrs {
                    match a {
                        NodeAttr::Owner(_) => {attr_map.insert("owner".to_string(), a.clone());}
                        NodeAttr::Label(_) => {attr_map.insert("label".to_string(), a.clone());}
                    }
                }
                node_attrs.insert(idx,attr_map);
            }
        }

        let node_count = next_idx;

        let mut edges = Vec::new();

        for item in &edge_lines {
            if let ParsedLine::Edge(from_id, to_id, formula) = item {
                let from = *id_map.get(from_id).unwrap();
                let to = *id_map.get(to_id).unwrap();

                let formula = match formula {
                Some(f) => f.clone(),
                None => Formula::True,
                };

                let available_at = |_: usize| true;
                edges.push(Edge::new(from, to, formula, available_at));
            }
        }

        TemporalGraph::new(
            node_count,
           // node_attrs,
            edges,
        )
    }
