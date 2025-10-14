use crate::temporal_graphs::TemporalGraph;

/// Computes the reachable set at time 0 for a punctual reachability game
/// by simple back propagation from the target set at time k.
///
/// # Arguments
/// * `graph` - Reference to the temporal graph
/// * `k` - The time horizon (time at which to reach the target)
/// * `player` - Boolean player who wants to reach (0 or 1)
/// * `target` - target set)
///
/// # Returns
/// A vector of booleans indicating which nodes are in the winning set at time 0
pub fn reachable_at(
    graph: &TemporalGraph,
    k: usize,
    player: bool,
    target: &Vec<bool>,
) -> Vec<bool> {
    // get node ownership from the graph
    let owner: Vec<bool> = graph.node_ownership();

    // w is the winning set at time k
    let mut wins_at: Vec<bool> = target.to_vec();
    //dbg!("target: {:?}", wins_at);

    // auxiliary variable for winning set at time i-1
    let mut wins_before: Vec<bool> = vec![false; graph.node_count];

    // compute wins_at one at a time from k-1 down to 0
    for i in (0..k).rev() {
        // wins_before = 1-step attractor of wins_at
        for node in graph.nodes() {
            //let successors: Vec<_> = graph.successors_at(node, i).collect();
            // dbg!(
            //     "SUCCS from {} (owner {}) at {} = {:?}",
            //     node, owner[node], i, &successors
            // );
            match owner[node] == player {
                true => wins_before[node] = graph.successors_at(node, i).any(|s| wins_at[s]),
                false => {
                    wins_before[node] = graph.successors_at(node, i).next().is_some()
                        && graph.successors_at(node, i).all(|s| wins_at[s])
                }
           }
        }
        wins_at = wins_before.clone();
        //dbg!("{:?}", wins_at);
        //dbg!("W_{} = {:?}", i, graph.ids_from_nodes_vec(&wins_at));
    }

    wins_at
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formulae::Formula;
    use crate::parser::NodeAttr;
    use crate::temporal_graphs::Edge;
    use std::collections::HashMap;

    // Helper function to create a single-state graph owned by player 0 with a self-loop
    // Creates: s0 (player 0) with self-loop edge that is always available (constraint "true")
    fn create_self_loop() -> TemporalGraph {
        let node_count = 1;

        // Create node ID mapping
        let mut node_id_map = HashMap::new();
        node_id_map.insert("s0".to_string(), 0);

        // Create node attributes
        let mut node_attrs = HashMap::new();
        let mut s0_attrs = HashMap::new();
        s0_attrs.insert("owner".to_string(), NodeAttr::Owner(false)); // player 0
        s0_attrs.insert("label".to_string(), NodeAttr::Label("s0".to_string()));
        node_attrs.insert(0, s0_attrs);

        // Create self-loop edge with constraint "true"
        let edges = vec![Edge::new(0, 0, Formula::True)];

        TemporalGraph::new(node_count, node_id_map, node_attrs, edges)
    }

    // Helper: two-state graph, both with self-loops (constraint true),
    // and state 0 has an edge to state 1 with constraint x >= 5.
    fn create_two_state_graph() -> TemporalGraph {
        let node_count = 2;
        let mut node_id_map = HashMap::new();
        node_id_map.insert("s0".to_string(), 0);
        node_id_map.insert("s1".to_string(), 1);

        let mut node_attrs = HashMap::new();
        let mut s0_attrs = HashMap::new();
        s0_attrs.insert("owner".to_string(), NodeAttr::Owner(false));
        s0_attrs.insert("label".to_string(), NodeAttr::Label("s0".to_string()));
        node_attrs.insert(0, s0_attrs);
        let mut s1_attrs = HashMap::new();
        s1_attrs.insert("owner".to_string(), NodeAttr::Owner(false));
        s1_attrs.insert("label".to_string(), NodeAttr::Label("s1".to_string()));
        node_attrs.insert(1, s1_attrs);

        use crate::formulae::{Expr, Formula};
        let edges = vec![
            // self-loops
            Edge::new(0, 0, Formula::True),
            Edge::new(1, 1, Formula::True),
            // edge from 0 to 1 with constraint x >= 5
            Edge::new(
                0,
                1,
                Formula::Ge(
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::Const(5)),
                ),
            ),
        ];
        TemporalGraph::new(node_count, node_id_map, node_attrs, edges)
    }

    #[test]
    fn test_self_loop_0() {
        let graph = create_self_loop();

        // Target the single state (node 0) at time 0
        let target = vec![true]; // node 0 is the target
        let k = 0;

        assert_eq!(reachable_at(&graph, k, true, &target), vec![true]);
        assert_eq!(reachable_at(&graph, k, false, &target), vec![true]);
    }

    #[test]
    fn test_self_loop_1() {
        let graph = create_self_loop();

        // Target the single state (node 0) at time 0
        let target = vec![true]; // node 0 is the target
        let k = 1;

        assert_eq!(reachable_at(&graph, k, true, &target), vec![true]);
        assert_eq!(reachable_at(&graph, k, false, &target), vec![true]);
    }

    #[test]
    fn test_two_state_reachability() {
        let graph = create_two_state_graph();

        // Let state 1 be the only target
        let target = vec![false, true];

        // assume perspective of player false
        let reacher = false;

        // player false can force to reach the target at time 0 only from the target
        assert_eq!(reachable_at(&graph, 0, reacher, &target), vec![false, true]);
        // player false can force to reach the target at times 1-4 only from the target
        assert_eq!(reachable_at(&graph, 1, reacher, &target), vec![false, true]);
        assert_eq!(reachable_at(&graph, 2, reacher, &target), vec![false, true]);
        assert_eq!(reachable_at(&graph, 3, reacher, &target), vec![false, true]);
        assert_eq!(reachable_at(&graph, 4, reacher, &target), vec![false, true]);

        // player false can force to reach the target at times 5 only from the target,
        // because it would have to take the edge 0 --> 1 at time 4;
        // it is only available from time 5 onwards.

        assert_eq!(reachable_at(&graph, 5, reacher, &target), vec![false, true]);

        // player false CAN force to reach the target at time 6 and later
        // from states 1 (target) AND 0
        // (by wating at 0 and then taking edge 0 --> 1 at time 5)
        assert_eq!(reachable_at(&graph, 6, reacher, &target), vec![true, true]);
        assert_eq!(reachable_at(&graph, 7, reacher, &target), vec![true, true]);

        // player !reacher == true (the opponent here) can force to reach the
        // target only from the target, no matter when, because she does not control the edges (own
        // state 0 in particular)
        assert_eq!(
            reachable_at(&graph, 7, !reacher, &target),
            vec![false, true]
        );
    }
}
