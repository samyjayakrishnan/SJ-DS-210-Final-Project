use std::collections::HashMap;

pub type NodeId = u32;
pub type AdjacencyList = Vec<HashMap<NodeId, u32>>; // NodeId and count of emails exchanged

pub struct Graph {
    pub adj_list: AdjacencyList,
}

impl Graph {
    // Creates a new Graph with a specified number of nodes
    pub fn new(num_nodes: usize) -> Self {
        Self {
            adj_list: vec![HashMap::new(); num_nodes],
        }
    }

    // Adds or updates an edge between src and dest
    pub fn add_or_update_edge(&mut self, src: NodeId, dest: NodeId) {
        *self.adj_list[src as usize].entry(dest).or_insert(0) += 1;
        *self.adj_list[dest as usize].entry(src).or_insert(0) += 1;
    }

    // Returns the number of unique connections (degree) of a node
    pub fn connections(&self, node: NodeId) -> usize {
        if let Some(neighbors) = self.adj_list.get(node as usize) {
            neighbors.len()
        } else {
            0 // Return 0 if the node does not exist
        }
    }
}


