mod graph;
mod bfs;
mod centrality;
use graph::{Graph, NodeId};
use centrality::closeness_centrality;
use std::fs::File;
use std::io::{self, BufRead};

// top 10 nodes by number of connections
fn top_ten_nodes_by_connections(graph: &Graph) -> Vec<(NodeId, usize)> {
    let mut node_connections: Vec<(NodeId, usize)> = (0..graph.adj_list.len())
        .map(|node| (node as NodeId, graph.connections(node as NodeId)))
        .collect();

    //sort by number of connections in descending order
    node_connections.sort_by(|a, b| b.1.cmp(&a.1));

    //take the top 10 nodes
    node_connections.into_iter().take(10).collect()
}

fn main() -> io::Result<()> {
    let filename = "enron.txt";
    let mut graph = Graph::new(36692);

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if !line.starts_with('#') {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let from_node: NodeId = parts[0].parse().unwrap();
                let to_node: NodeId = parts[1].parse().unwrap();
                graph.add_or_update_edge(from_node, to_node);
            }
        }
    }

    // find the pair of nodes with the most emails
    let mut max_emails = 0;
    let mut max_pair = (0, 0);
    for (node, edges) in graph.adj_list.iter().enumerate() {
        for (&target, &count) in edges {
            if count > max_emails {
                max_emails = count;
                max_pair = (node as NodeId, target);
            }
        }
    }

    println!("Pair of nodes with most emails: {:?} ({} emails)", max_pair, max_emails);

    // calculate and print closeness centrality for a specific node
    let node_id = 273; 
    let centrality = closeness_centrality(&graph, node_id);
    println!("Closeness Centrality of node {}: {}", node_id, centrality);

    //find and print the top 10 nodes by number of connections
    let top_nodes_by_connections = top_ten_nodes_by_connections(&graph);
    println!("Top 10 Nodes by Number of Connections:");
    for (node, connections) in top_nodes_by_connections {
        println!("Node {}: {} connections", node, connections);
    }

    // calculate and print the top nodes by closeness centrality in range 5000-5055
    let start_node = 5000;
    let end_node = 5055;
    let mut centralities: Vec<(NodeId, f64)> = (start_node..=end_node)
        .map(|node_id| {
            let centrality = closeness_centrality(&graph, node_id as NodeId);
            (node_id as NodeId, centrality)
        })
        .collect();

    //sort by centrality in descending order
    centralities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // get the top nodes within the range
    let top_centralities = centralities.into_iter().take(10).collect::<Vec<_>>();

    println!("Top Nodes by Closeness Centrality in Range 5000-5055:");
    for (node, centrality) in top_centralities {
        println!("Node {}: Closeness Centrality = {}", node, centrality);
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    
    // small test graph for all tests
    fn create_test_graph() -> Graph {
        let mut graph = Graph::new(13);
        graph.add_or_update_edge(5, 6);
        graph.add_or_update_edge(6, 7);
        graph.add_or_update_edge(7, 8);
        graph.add_or_update_edge(8, 9);
        graph.add_or_update_edge(9, 10);
        graph.add_or_update_edge(10, 11);
        graph.add_or_update_edge(11, 12);
        graph.add_or_update_edge(12, 5);
        graph
    }

    #[test]
    fn test_connections() {
        let graph = create_test_graph();
        assert_eq!(graph.connections(5), 2); 
        assert_eq!(graph.connections(10), 2); 
    }

    #[test]
    fn test_closeness_centrality() {
        let graph = create_test_graph();
        let centrality = centrality::closeness_centrality(&graph, 5);
        assert!(centrality > 0.0);
    }
}
