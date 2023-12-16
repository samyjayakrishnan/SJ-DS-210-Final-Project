use crate::graph::{Graph, NodeId};
use crate::bfs::bfs;

pub fn closeness_centrality(graph: &Graph, node: NodeId) -> f64 {
    let distances = bfs(graph, node);

    // filter out unreachable nodes
    let sum_distances: u32 = distances.iter().filter(|&&d| d != u32::MAX).sum();

    if sum_distances > 0 {
        //number of reachable nodes
        let reachable_nodes = distances.iter().filter(|&&d| d != u32::MAX).count();
        reachable_nodes as f64 / sum_distances as f64
    } else {
        0.0 
    }
}

