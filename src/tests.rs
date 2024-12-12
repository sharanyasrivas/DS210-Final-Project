use my_project::graph::{SocialGraph};
use petgraph::graph::NodeIndex;

#[test]
fn test_graph_loading() {
    let graph = SocialGraph::load_graph("test_data.txt").unwrap();
    assert!(graph.graph.node_count() > 0, "Graph should contain nodes.");
    assert!(graph.graph.edge_count() > 0, "Graph should contain edges.");
}

#[test]
fn test_similarity_calculation() {
    let graph = SocialGraph::load_graph("test_data.txt").unwrap();
    let nodes: Vec<_> = graph.graph.node_indices().collect();
    if nodes.len() >= 2 {
        let similarity = graph.calculate_similarity(nodes[0], nodes[1]);
        assert!(
            (0.0..=1.0).contains(&similarity),
            "Similarity should be between 0 and 1."
        );
    }
}








