use std::fs::File;
use std::io::{BufRead,BufReader};


#[derive(Debug)]
pub struct StreamInteraction {
    pub user_id: i64,
    pub streamer_id: i64,
    pub streamer_username: String,
    pub time_start: i64,
    pub time_stop: i64,
}

// this functions reads a file and returns a vector of StreamInteraction structs
pub fn read_file(filename: &str) -> Vec<StreamInteraction> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let fields: Vec<&str> = line.split(',').collect();

        let interaction = StreamInteraction {
            user_id: fields[0].parse().unwrap(),
            streamer_id: fields[1].parse().unwrap(),
            streamer_username: fields[2].to_string(),
            time_start: fields[3].parse().unwrap(),
            time_stop: fields[4].parse().unwrap(),
        };
        data.push(interaction);
    }
    data
}