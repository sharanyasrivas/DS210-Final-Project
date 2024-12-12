mod graph;
mod utility;

use graph::SocialGraph;

fn main() {
    // Use `SocialGraph::load_graph` to call the associated function
    let graph = match SocialGraph::load_graph("twitter_combined.txt") {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error loading graph: {}", e);
            return;
        }
    };

    println!(
        "Graph loaded successfully. Nodes: {}, Edges: {}",
        graph.graph.node_count(),
        graph.graph.edge_count()
    );

    // Example: Use a utility function if needed
    utility::example_utility_function();
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}

use std::io;

fn get_user_input() -> (String, String) {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the first node:");
    io::stdin().read_line(&mut input1).unwrap();

    println!("Enter the second node:");
    io::stdin().read_line(&mut input2).unwrap();

    (input1.trim().to_string(), input2.trim().to_string())
}

fn run_program() -> io::Result<()> {
    // Use `SocialGraph::load_graph` to call the associated function
    let graph = match SocialGraph::load_graph("twitter_combined.txt") {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error loading graph: {}", e);
            return Err(e); // Return error if loading the graph fails
        }
    };

    println!(
        "Graph loaded successfully. Nodes: {}, Edges: {}",
        graph.graph.node_count(),
        graph.graph.edge_count()
    );

    // Example: Use a utility function if needed
    utility::example_utility_function();

    // Optionally call other methods on the `SocialGraph` object
    graph.find_extreme_similarity();

    Ok(())
}

