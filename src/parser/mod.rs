use lalrpop_util::lalrpop_mod;
use crate::temporal_graphs::NodeAttr;


#[derive(Debug)]
pub struct ParsedData {
    pub nodes: Vec<(String, Vec<NodeAttr>)>,
    pub edges: Vec<(String, String, Vec<(String, String)>)>,
}


#[derive(Debug)]
pub enum ParsedLine {
    Node(String, Vec<NodeAttr>),
    Edge(String, String, Vec<(String, String)>),
    Empty,
}

lalrpop_mod!(pub tg_parser, "/parser/tg_parser.rs"); // LALRPOP parser module
//lalrpop_mod!(formula_parser);
