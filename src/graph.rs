use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

/// Struct to represent a social graph.
pub struct SocialGraph {
    pub graph: Graph<(), (), Undirected>, // The graph itself
    pub node_map: HashMap<String, NodeIndex>, // Maps node names to NodeIndex
}

impl SocialGraph {
    /// Load a graph from a file.
    pub fn load_graph(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        let mut graph = Graph::<(), (), Undirected>::new_undirected();
        let mut node_map = HashMap::new();

        for line in reader.lines() {
            if let Ok(edge_line) = line {
                let nodes: Vec<&str> = edge_line.split_whitespace().collect();
                if nodes.len() == 2 {
                    let a = nodes[0].to_string();
                    let b = nodes[1].to_string();

                    let node_a = *node_map.entry(a).or_insert_with(|| graph.add_node(()));
                    let node_b = *node_map.entry(b).or_insert_with(|| graph.add_node(()));
                    graph.add_edge(node_a, node_b, ());
                }
            }
        }

        Ok(SocialGraph {
            graph,
            node_map,
        })
    }

    /// Find the nodes with extreme similarities in the graph.
    pub fn find_extreme_similarity(&self) {
        let mut max_similarity = 0.0;
        let mut min_similarity = 1.0;
        let mut max_nodes = (NodeIndex::new(0), NodeIndex::new(0));
        let mut min_nodes = (NodeIndex::new(0), NodeIndex::new(0));
    
        for node1 in self.graph.node_indices() {
            for node2 in self.graph.node_indices() {
                if node1 != node2 {
                    let similarity = self.calculate_similarity(node1, node2);
                    if similarity > max_similarity {
                        max_similarity = similarity;
                        max_nodes = (node1, node2);
                    }
                    if similarity < min_similarity {
                        min_similarity = similarity;
                        min_nodes = (node1, node2);
                    }
                }
            }
        }
    
        println!(
            "Max similarity: {} between nodes {:?} and {:?}",
            max_similarity, max_nodes.0, max_nodes.1
        );
        println!(
            "Min similarity: {} between nodes {:?} and {:?}",
            min_similarity, min_nodes.0, min_nodes.1
        );
    }

    /// Calculate similarity between two nodes based on common neighbors.
    pub fn calculate_similarity(&self, node1: NodeIndex, node2: NodeIndex) -> f64 {
        let neighbors1: Vec<NodeIndex> = self.graph.neighbors(node1).collect();
        let neighbors2: Vec<NodeIndex> = self.graph.neighbors(node2).collect();

        // Calculate the intersection of the two neighbor lists
        let common_neighbors: Vec<NodeIndex> = neighbors1
            .iter()
            .filter(|&&n| neighbors2.contains(&n))
            .copied()
            .collect();

        // Return similarity as the number of common neighbors divided by the total number of unique neighbors
        let total_neighbors = neighbors1.len() + neighbors2.len() - common_neighbors.len();
        if total_neighbors == 0 {
            return 0.0; // No similarity if there are no neighbors
        }

        common_neighbors.len() as f64 / total_neighbors as f64
    }
}

   
