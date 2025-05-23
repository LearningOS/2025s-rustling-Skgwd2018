/*
	graph
	This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

/// 无向图
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph { adjacency_table: HashMap::new() }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node1, node2, weight) = edge;
        // 确保两个节点都存在
        self.add_node(node1);
        self.add_node(node2);

        // 将node2添加到node1的neighbors
        self.adjacency_table_mutable().get_mut(node1).unwrap().push((node2.to_string(), weight));
        // 将node1添加到node2的neighbors中(无向的)
        self.adjacency_table_mutable().get_mut(node2).unwrap().push((node1.to_string(), weight));
    }
}

/// 图
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        if self.contains(node) {
            false
        } else {
            self.adjacency_table_mutable().insert(node.to_string(), Vec::new());
            true
        }
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node1, node2, weight) = edge;
        // 确保两个节点都存在(若不存在则添加)
        self.add_node(node1);
        self.add_node(node2);

        let adj_table = self.adjacency_table_mutable();

        // 检查是否已存在 node1 -> node2 的边
        if !adj_table[node1].iter().any(|(n, _)| n == node2) {
            // 添加 node2 到 node1 的neighbors列表
            adj_table.get_mut(node1).unwrap().push((node2.to_string(), weight));
            // 添加 node1 到 node2 的neighbors列表(无向图需要双向)
            adj_table.get_mut(node2).unwrap().push((node1.to_string(), weight));
        }
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }

        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
