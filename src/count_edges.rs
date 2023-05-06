use std::collections::HashMap;
use crate::graph::Graph;

// this function calculates the in-degree for each node in the graph
pub fn calculate_indegree(graph: &HashMap<String, Graph>) -> HashMap<String, usize> {
    let mut indegree_map: HashMap<String, usize> = HashMap::new();

    for (streamer_username, node) in graph.iter() {
        for _edge in node.in_edges.iter() {
            *indegree_map.entry(streamer_username.clone()).or_insert(0) += 1;
        }
    }

    indegree_map
}

// this function finds the node with the most amount of edges pointing towards it
// the most popular streamer from the dataset will be the streamer with the most interactions
pub fn find_most_popular_streamer(indegree_map: &HashMap<String, usize>) -> Option<String> {
    let binding = "".to_string();
    let (streamer_username, _) = indegree_map
        .iter()
        .max_by_key(|&(_, indegree)| indegree)
        .unwrap_or((&binding, &0));
    
    if streamer_username.is_empty(){
        None
    }else{
        Some(streamer_username.clone())
    }
}