mod graph;
mod utility;

use graph::SocialGraph;
use petgraph::graph::NodeIndex;

fn main() {
    // Use `SocialGraph::load_graph` to call the associated function
    let graph = match SocialGraph::load_graph("twitter_combined.txt") {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error loading graph: {}", e);
            return;
        }
    };

    // Print graph information
    println!(
        "Graph loaded successfully. Nodes: {}, Edges: {}",
        graph.graph.node_count(),
        graph.graph.edge_count()
    );

    // Print first few neighbors of a node to ensure the graph has connections
    println!("First few nodes and their neighbors:");
    let first_node = graph.graph.node_indices().next().unwrap(); // Get the first node
    let neighbors: Vec<NodeIndex> = graph.graph.neighbors(first_node).collect(); // Collect as Vec<NodeIndex>
    for neighbor in neighbors.iter().take(5) {
        println!("Node {:?} has neighbor {:?}", first_node, neighbor);
    }

    // Example: Use a utility function if needed
    utility::example_utility_function();

    // Call the similarity function to calculate and print the max and min similarity
    graph.find_extreme_similarity();

    // Optionally, test similarity between two specific nodes
    let node1 = graph.graph.node_indices().next().unwrap(); // Get the first node
    let node2 = graph.graph.node_indices().skip(1).next().unwrap(); // Get the second node
    let similarity = graph.calculate_similarity(node1, node2);
    println!(
        "Similarity between node {:?} and node {:?}: {}",
        node1, node2, similarity
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}

