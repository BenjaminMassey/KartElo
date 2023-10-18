use serde::{Deserialize, Serialize};
use std::{fs, collections::HashMap};

use crate::kart::*;

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    matches: Vec<Match>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Match {
    game_type: String,
    players: Vec<String>,
    scores: Vec<f32>,
}

pub fn load_content(mut players: HashMap<String, f32>) -> HashMap<String, f32> {
    // https://stackoverflow.com/q/63657897
    let path = "./content.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: Content = serde_json::from_str(&data).expect("Unable to parse");
    for mat in &res.matches {
        for player in &mat.players {
            if !players.contains_key(player) {
                players.insert(player.to_owned(), 1000f32);
            }
        }
        if !matches!(mat.game_type.as_ref(), "one_on_one" | "free_for_all") {
            panic!("Had unknown match type \"{}\"", mat.game_type);
        }
            
        let mut entrants: Vec<(&str, f32)> = Vec::new();
        for i in 0..mat.players.len() {
            entrants.push((&mat.players[i], mat.scores[i]));
        }

        free_for_all(&mut players, entrants);
    }
    players
}
