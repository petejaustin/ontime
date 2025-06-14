use std::collections::HashSet;
use std::collections::HashMap;

pub type Node = usize;

/// Trait representing a temporal edge in a directed graph.
/// Implementors must provide methods to access the source and target nodes,
/// and to determine if the edge is available at a given time step.
pub trait TemporalEdge {
    /// Returns the source node of the edge.
    fn source(&self) -> &Node;
    /// Returns the target node of the edge.
    fn target(&self) -> &Node;
    /// Returns true if the edge is available at the given time.
    fn is_available(&self, time: usize) -> bool;
}


/// An explicit temporal edge with a set of available time steps.
/// Stores the source and target nodes, and a set of time steps (usize)
/// indicating when the edge is available.
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct ExplicitTemporalEdge {
    source: Node,
    target: Node,
    availability: HashSet<usize>,
}

impl ExplicitTemporalEdge {
    pub fn new(source: Node, target: Node, availability: HashSet<usize>) -> Self {
        Self { source, target, availability }
    }
}

/// Implementation of the TemporalEdge trait for ExplicitTemporalEdge.
/// Provides access to the source and target nodes, and checks availability
/// at a given time step by querying the internal HashSet.
impl TemporalEdge for ExplicitTemporalEdge {
    /// Returns the source node of the edge.
    fn source(&self) -> &Node {
        &self.source
    }
    /// Returns the target node of the edge.
    fn target(&self) -> &Node {
        &self.target
    }
    /// Returns true if the edge is available at the given time.
    fn is_available(&self, time: usize) -> bool {
        self.availability.contains(&time)
    }
}

/// A temporal graph is parameterized by the type of TemporalEdge.
/// Stores outgoing edges for each node for efficient access.
pub struct TemporalGraph<E: TemporalEdge> {
    /// The number of nodes in the graph.
    pub node_count: usize,
    /// A map from node to its outgoing edges.
    pub edges: HashMap<Node, Vec<E>>,
}

impl<E: TemporalEdge> TemporalGraph<E> {
    /// Creates a new TemporalGraph from a node count and a list of edges.
    pub fn new(node_count: Node, edges: Vec<E>) -> Self {
        let mut edge_map: HashMap<Node, Vec<E>> = HashMap::new();
        for edge in edges {
            edge_map.entry(*edge.source()).or_default().push(edge);
        }
        Self { node_count, edges: edge_map }
    }

    /// Returns an iterator over all edges in the graph.
    pub fn edges(&self) -> impl Iterator<Item = &E> {
        self.edges.values().flat_map(|v| v.iter())
    }

    /// Returns an iterator over all edges starting from the given node.
    pub fn edges_from(&self, from: Node) -> impl Iterator<Item = &E> {
        self.edges.get(&from).into_iter().flat_map(|v| v.iter())
    }

    /// Returns an iterator over all outgoing edges from the given node that are available at the given time.
    pub fn edges_from_at(&self, from: Node, time: usize) -> impl Iterator<Item = &E> {
        self.edges_from(from).filter(move |e| e.is_available(time))
    }

    /// Returns an iterator over all node indices in the graph.
    pub fn nodes(&self) -> impl Iterator<Item = Node> {
        0..self.node_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_explicit_temporal_graph() -> (TemporalGraph<ExplicitTemporalEdge>, Vec<ExplicitTemporalEdge>) {
        let mut avail1 = HashSet::new();
        avail1.insert(1);
        avail1.insert(2);

        let mut avail2 = HashSet::new();
        avail2.insert(2);
        avail2.insert(3);

        let edge1 = ExplicitTemporalEdge::new(0, 1, avail1.clone());
        let edge2 = ExplicitTemporalEdge::new(1, 2, avail2.clone());

        let edges = vec![edge1, edge2];
        let graph = TemporalGraph::new(3, edges.clone());
        (graph, edges)
    }

    #[test]
    fn test_nodes_iterator() {
        let (graph, _) = build_explicit_temporal_graph();
        let nodes: Vec<Node> = graph.nodes().collect();
        assert_eq!(nodes, vec![0, 1, 2]);
    }

    #[test]
    fn test_edges_iterator() {
        let (graph, edges) = build_explicit_temporal_graph();
        let mut all_edges: Vec<&ExplicitTemporalEdge> = graph.edges().collect();
        let mut expected_edges: Vec<&ExplicitTemporalEdge> = edges.iter().collect();
        all_edges.sort_by_key(|e| (e.source(), e.target()));
        expected_edges.sort_by_key(|e| (e.source(), e.target()));
        assert_eq!(all_edges, expected_edges);
    }

    #[test]
    fn test_edges_from_node_0() {
        let (graph, edges) = build_explicit_temporal_graph();
        let from_0: Vec<&ExplicitTemporalEdge> = graph.edges_from(0).collect();
        assert_eq!(from_0.len(), 1);
        assert_eq!(from_0[0], &edges[0]);
        assert!(from_0[0].is_available(1));
        assert!(from_0[0].is_available(2));
        assert!(!from_0[0].is_available(3));
    }

    #[test]
    fn test_edges_from_node_1() {
        let (graph, edges) = build_explicit_temporal_graph();
        let from_1: Vec<&ExplicitTemporalEdge> = graph.edges_from(1).collect();
        assert_eq!(from_1.len(), 1);
        assert_eq!(from_1[0], &edges[1]);
        assert!(from_1[0].is_available(2));
        assert!(from_1[0].is_available(3));
        assert!(!from_1[0].is_available(1));
    }

    #[test]
    fn test_edges_from_node_2() {
        let (graph, _) = build_explicit_temporal_graph();
        let from_2: Vec<&ExplicitTemporalEdge> = graph.edges_from(2).collect();
        assert!(from_2.is_empty());
    }


    #[test]
    fn test_edges_from_at() {
        let (graph, edges) = build_explicit_temporal_graph();

        // At time 1, only the edge from 0 to 1 is available
        let from_0_at_1: Vec<&ExplicitTemporalEdge> = graph.edges_from_at(0, 1).collect();
        assert_eq!(from_0_at_1, vec![&edges[0]]);

        // At time 2, both edges are available from their respective sources
        let from_0_at_2: Vec<&ExplicitTemporalEdge> = graph.edges_from_at(0, 2).collect();
        assert_eq!(from_0_at_2, vec![&edges[0]]);
        let from_1_at_2: Vec<&ExplicitTemporalEdge> = graph.edges_from_at(1, 2).collect();
        assert_eq!(from_1_at_2, vec![&edges[1]]);

        // At time 3, only the edge from 1 to 2 is available
        let from_1_at_3: Vec<&ExplicitTemporalEdge> = graph.edges_from_at(1, 3).collect();
        assert_eq!(from_1_at_3, vec![&edges[1]]);
        let from_0_at_3: Vec<&ExplicitTemporalEdge> = graph.edges_from_at(0, 3).collect();
        assert!(from_0_at_3.is_empty());
    }
}
