use rustc_hash::FxHashMap;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

pub struct Edge<NodeId> {
    pub dest: NodeId,
    pub cost: u32,
}

pub struct Node<NodeId> {
    edges: Vec<Edge<NodeId>>,
}

impl<NodeId> Node<NodeId> {
    pub fn add_edge(&mut self, dest: NodeId, cost: u32) {
        self.edges.push(Edge { dest, cost });
    }
}

pub struct Graph<NodeId> {
    nodes: FxHashMap<NodeId, Node<NodeId>>,
}

impl<NodeId> Graph<NodeId>
where
    NodeId: Eq + Hash,
    NodeId: Ord + PartialOrd,
    NodeId: Clone,
{
    pub fn add_node(&mut self, id: NodeId) -> Option<Node<NodeId>> {
        self.nodes.insert(id, Node { edges: Vec::new() })
    }

    pub fn get_node_mut(&mut self, id: &NodeId) -> Option<&mut Node<NodeId>> {
        self.nodes.get_mut(id)
    }

    pub fn dijkstra(&self, start_node: &NodeId, end_node: &NodeId) -> Option<Vec<(NodeId, u32)>> {
        if !self.nodes.contains_key(start_node) || !self.nodes.contains_key(end_node) {
            return None;
        }

        // Cumulative, minumum cost of moving from start_node node to Node
        let mut total_costs: FxHashMap<NodeId, u32> = FxHashMap::default();

        // Node (value) we came from when moving to Node (key)
        let mut previous: FxHashMap<NodeId, NodeId> = FxHashMap::default();

        // Priority queue of sorted by lowest cost (followed by NodeId in case of a tie)
        let mut prio_queue = BinaryHeap::new();
        prio_queue.push(Reverse((0, start_node.clone())));

        while let Some(Reverse((current_cost, current_node))) = prio_queue.pop() {
            if &current_node == end_node {
                let mut reverse_path = vec![];
                let mut node = end_node;
                while node != start_node {
                    reverse_path.push((node.clone(), *total_costs.get(node).unwrap()));
                    node = previous.get(node).unwrap();
                }
                reverse_path.push((start_node.clone(), 0));
                reverse_path.reverse();
                return Some(reverse_path);
            }

            for edge in self
                .nodes
                .get(&current_node)
                .expect("invalid edge")
                .edges
                .iter()
            {
                let proposed_cost = current_cost + edge.cost;
                let best_cost_so_far = total_costs.entry(edge.dest.clone()).or_insert(u32::MAX);
                if proposed_cost < *best_cost_so_far {
                    prio_queue.push(Reverse((proposed_cost, edge.dest.clone())));
                    *best_cost_so_far = proposed_cost;
                    previous.insert(edge.dest.clone(), current_node.clone());
                }
            }
        }

        None
    }
}

impl<NodeId> Default for Graph<NodeId> {
    fn default() -> Self {
        Graph {
            nodes: FxHashMap::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        //   start        c
        //       | \4  3/   \2
        //       |  \  /     \
        //       4    b --6---end
        //       |  /  \     /
        //       | /2  1\   /3
        //       a        d
        let mut digraph: Graph<&str> = Graph::default();
        digraph.add_node("start");
        digraph.add_node("a");
        digraph.add_node("b");
        digraph.add_node("c");
        digraph.add_node("d");
        digraph.add_node("end");

        let node = digraph.get_node_mut(&"start").unwrap();
        node.add_edge("a", 4);
        node.add_edge("b", 4);

        let node = digraph.get_node_mut(&"a").unwrap();
        node.add_edge("start", 4);
        node.add_edge("b", 2);

        let node = digraph.get_node_mut(&"b").unwrap();
        node.add_edge("start", 4);
        node.add_edge("a", 2);
        node.add_edge("c", 3);
        node.add_edge("d", 1);
        node.add_edge("end", 6);

        let node = digraph.get_node_mut(&"c").unwrap();
        node.add_edge("b", 3);
        node.add_edge("end", 2);

        let node = digraph.get_node_mut(&"d").unwrap();
        node.add_edge("b", 1);
        node.add_edge("end", 3);

        let node = digraph.get_node_mut(&"end").unwrap();
        node.add_edge("c", 2);
        node.add_edge("b", 6);
        node.add_edge("d", 3);

        assert_eq!(
            digraph.dijkstra(&"start", &"end"),
            Some(vec![("start", 0), ("b", 4), ("d", 5), ("end", 8)])
        );
    }
}
