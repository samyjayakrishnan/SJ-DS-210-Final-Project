use crate::graph::{Graph, NodeId};
use std::collections::VecDeque;

pub fn bfs(graph: &Graph, start: NodeId) -> Vec<u32> {
    let num_nodes = graph.adj_list.len();
    let mut distances = vec![u32::MAX; num_nodes]; 
    let mut queue = VecDeque::new();

    distances[start as usize] = 0;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for (&neighbor, _) in graph.adj_list[node as usize].iter() { 
            if distances[neighbor as usize] == u32::MAX {
                queue.push_back(neighbor);
                distances[neighbor as usize] = distances[node as usize] + 1;
            }
        }
    }

    distances
}
