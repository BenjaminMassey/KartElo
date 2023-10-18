use serde::{Deserialize, Serialize};
use std::{fs, collections::HashMap};

use crate::kart::*;

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    matches: Vec<HashMap<String, f32>>,
}

pub fn load_content(mut players: HashMap<String, f32>) -> HashMap<String, f32> {
    // https://stackoverflow.com/q/63657897
    let path = "./content.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: Content = serde_json::from_str(&data).expect("Unable to parse");
    for mat in &res.matches {
        for player in mat.keys() {
            if !players.contains_key(player) {
                players.insert(player.to_owned(), 1000f32);
            }
        }
        free_for_all(&mut players, mat);
    }
    players
}
