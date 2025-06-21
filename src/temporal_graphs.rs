use std::collections::{HashMap, HashSet};

use crate::{formulae::Formula, parser::NodeAttr};

#[allow(dead_code)]
pub type Node = usize;

#[allow(dead_code)]
pub struct Edge {
    source: Node,
    target: Node,
    formula: Formula,
    available_at: Box<dyn Fn(usize) -> bool + 'static>,
}

impl Edge {
    pub fn new(source: Node, target: Node, formula: Formula) -> Self {
        let available_at = match formula.clone().as_closure() {
            Ok(f) => f,
            Err(_) => Box::new(|_| false),
        };
        Self {
            source,
            target,
            formula,
            available_at,
        }
    }
    pub fn new_simple(source: Node, target: Node) -> Self {
        Self::new(source, target, Formula::True)
    }

    fn source(&self) -> &Node {
        &self.source
    }
    fn target(&self) -> &Node {
        &self.target
    }
    fn is_available(&self, time: usize) -> bool {
        (self.available_at)(time)
    }
}
// to print Edges : skip available_at
impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Edge")
            .field("source", &self.source)
            .field("target", &self.target)
            .field("formula", &self.formula)
            .finish()
    }
}

/// A temporal graph is parameterized by the type of TemporalEdge.
/// Stores outgoing edges for each node for efficient access.
/// Stores outgoing edges for each node for efficient access.
#[derive(Debug)]
pub struct TemporalGraph {
    /// The number of nodes in the graph.
    pub node_count: usize,
    /// A map from node to its outgoing edges.
    pub edges: HashMap<Node, Vec<Edge>>,
    // Map from node to its attributes
    pub node_attrs: HashMap<Node, HashMap<String, NodeAttr>>,

    /// Map node ids to their index
    pub node_id_map: HashMap<String, Node>,
}
impl TemporalGraph {
    /// Creates a new TemporalGraph from a node count and a list of edges.
    pub fn new(
        node_count: Node,
        node_id_map: HashMap<String, Node>,
        node_attrs: HashMap<Node, HashMap<String, NodeAttr>>,
        edges: Vec<Edge>,
    ) -> Self {
        let mut edge_map: HashMap<Node, Vec<Edge>> = HashMap::new();
        for edge in edges {
            edge_map.entry(*edge.source()).or_default().push(edge);
        }
        Self {
            node_count,
            node_id_map,
            node_attrs,
            edges: edge_map,
        }
    }

    /// Returns an iterator over all edges in the graph.
    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.values().flat_map(|v| v.iter())
    }

    /// Returns an iterator over all edges starting from the given node.
    pub fn edges_from(&self, from: Node) -> impl Iterator<Item = &Edge> {
        self.edges.get(&from).into_iter().flat_map(|v| v.iter())
    }

    /// Returns an iterator over all outgoing edges from the given node that are available at the given time.
    pub fn edges_from_at(&self, from: Node, time: usize) -> impl Iterator<Item = &Edge> {
        self.edges_from(from).filter(move |e| e.is_available(time))
    }

    /// Returns an iterator over all node indices in the graph.
    pub fn nodes(&self) -> impl Iterator<Item = Node> {
        0..self.node_count
    }

    pub fn successors_at(&self, from: Node, time: usize) -> impl Iterator<Item = Node> {
        self.edges_from_at(from, time).map(|e| *e.target())
    }
    
    pub fn node_ownership(&self) -> Vec<bool> {
        let mut player_one_nodes = vec![false; self.node_count];
        for node in self.nodes() {
            player_one_nodes[node] = self.node_attrs.get(&node)
                .and_then(|attrs| attrs.get("owner"))
                .and_then(|attr| match attr {
                    NodeAttr::Owner(val) => Some(*val),
                    _ => None,
                })
                .unwrap_or(false)

        }
        player_one_nodes
    }

    /// Given a set of node id strings, returns a vector of bools of length node_count.
    /// For each string, if node_id_map gives a Node with index n, then the returned vector is true at position n.
    pub fn nodes_selected_from_ids(&self, ids: &HashSet<String>) -> Vec<bool> {
        let mut selected = vec![false; self.node_count];
        for id in ids {
            if let Some(&n) = self.node_id_map.get(id) {
                if n < self.node_count {
                    selected[n] = true;
                }
            }
        }
        selected
    }

    // id strings for vector of nodes
    pub fn ids_from_nodes_vec(&self, v: &[bool]) -> HashSet<String>{
        let mut ids = HashSet::<String>::new();
        for (id, &idx) in &self.node_id_map {
            if idx < v.len() && v[idx] {
                ids.insert(id.clone());
            }
        }
        ids
    }
}

