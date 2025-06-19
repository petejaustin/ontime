use std::collections::HashMap;

use crate::formulae::Formula;

#[allow(dead_code)]
pub type Node = usize;


#[derive(Debug)]
#[allow(dead_code)]
pub struct Edge {
    source: Node,
    target: Node,
    formula: Formula,
    available_at: fn(usize) -> bool,
}

impl Edge {
    pub fn new(source: Node, target: Node, formula:Formula, available_at: fn(usize) -> bool) -> Self {
        Self {
            source,
            target,
            formula,
            available_at: available_at,
        }
    }
    pub fn new_simple(source: Node, target: Node) -> Self {
        Self {
            source,
            target,
            formula: Formula::True,
            available_at: |_| true,
        }
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
    //pub node_attrs: HashMap<Node, HashMap<String, NodeAttr>>,
}
impl TemporalGraph {
    /// Creates a new TemporalGraph from a node count and a list of edges.
    pub fn new(
        node_count: Node,
        //node_attrs: HashMap<Node, HashMap<String, NodeAttr>>,
        edges: Vec<Edge>,
    ) -> Self {
        let mut edge_map: HashMap<Node, Vec<Edge>> = HashMap::new();
        for edge in edges {
            edge_map.entry(*edge.source()).or_default().push(edge);
        }
        Self {
            node_count,
            //node_attrs,
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
}
