use std::collections::HashMap;
use crate::read_data::StreamInteraction;

#[derive(Debug)]
pub struct Graph{
    pub name: String,
    pub in_edges: Vec<i64>,
}

// creates a graph from a vector of stream interactions
pub fn create_graph(data: &Vec<StreamInteraction>) -> HashMap<String, Graph> {
    let mut graph = HashMap::new();

    for interaction in data {
        let streamer_username = interaction.streamer_username.clone();

        let node = graph.entry(streamer_username.clone()).or_insert_with(|| Graph {
            name: streamer_username.clone(),
            in_edges: Vec::new(),
        });

        node.in_edges.push(interaction.user_id);
    }

    graph
}