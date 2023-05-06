mod read_data;
mod graph;
mod count_edges;

use read_data::read_file;
use graph::create_graph;
use count_edges::{calculate_indegree, find_most_popular_streamer};

fn main() {
    let data = read_file("100k_a.csv");
    let graph = create_graph(&data);
    let interactions = calculate_indegree(&graph);
    let most_pop_streamer = find_most_popular_streamer(&interactions);

    // prints the number of nodes in the graph
    println!("Number of streamers: {}", graph.len());

    match most_pop_streamer {
        Some(streamer_username) => {
            let degree = interactions.get(&streamer_username).unwrap();
            println!("The most popular streamer is {}!", streamer_username);
            println!("{} had {} interactions with their viewers", streamer_username, degree);
        },
        None => {
            println!("No streamers found");
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let data = read_file("test_data.txt");
        let graph = create_graph(&data);

        assert_eq!(data.len(), 12);

        // ensure that the program correctly identifies the number of unique streamers
        assert_eq!(graph.len(), 5);
    }

    #[test]
    fn test_calculate_indegree() {
        let data = read_file("test_data.txt");
        let graph = create_graph(&data);
        let interactions = calculate_indegree(&graph);

        // ensure that the function correctly counts the number of edges pointing toward each node
        assert_eq!(interactions.get("streamer1"), Some(&2));
        assert_eq!(interactions.get("streamer2"), Some(&2));
        assert_eq!(interactions.get("streamer3"), Some(&5));
        assert_eq!(interactions.get("streamer4"), Some(&2));
        assert_eq!(interactions.get("streamer5"), Some(&1));
        assert_eq!(interactions.get("streamer6"), None);
    }

    #[test]
    fn test_most_pop_streamer() {
        let data = read_file("test_data.txt");
        let graph = create_graph(&data);
        let interactions = calculate_indegree(&graph);
        let most_popular_streamer = find_most_popular_streamer(&interactions);
     
        // ensures that the funtion correctly outputs the node with the most edges pointing toward it
        assert_eq!(most_popular_streamer, Some("streamer3".to_string()));
    }
}
