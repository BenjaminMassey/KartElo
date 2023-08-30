use serde::{Deserialize, Serialize};
use std::fs;

use crate::kart::*;

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    players: Vec<String>,
    matches: Vec<Match>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Match {
    game_type: String,
    players: Vec<String>,
    scores: Vec<f32>,
}

pub fn load_content(mut players: Vec<Player>) -> Vec<Player> {
    // https://stackoverflow.com/q/63657897
    let path = "./content.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: Content = serde_json::from_str(&data).expect("Unable to parse");
    for player in &res.players {
        players.push(Player::new(player));
    }
    for mat in &res.matches {
        if mat.game_type == "one_on_one" {
            players = one_on_one(
                players,
                &mat.players[0],
                &mat.players[1],
                mat.scores[0],
                mat.scores[1]
            );
        } else if mat.game_type == "free_for_all"  {
            let mut entrants: Vec<(&str, f32)> = Vec::new();
            for i in 0..mat.players.len() {
                entrants.push((&mat.players[i], mat.scores[i]));
            }
            players = free_for_all(players, entrants);
        } else {
            panic!("Had unknown match type \"{}\"", mat.game_type);
        }
    }
    players
}