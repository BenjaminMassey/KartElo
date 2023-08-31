use std::fs;
use std::collections::HashMap;

fn top_players(players: &HashMap<String, f32>) -> Vec<(String, String)> {
    let mut sortable: Vec<(&String, &f32)> = players.iter().collect();
    sortable.sort_by(|a, b| (*b.1 as i32).cmp(&(*a.1 as i32)));
    
    let mut result: Vec<(String, String)> = Vec::new();
    for player in sortable {
        result.push((player.0.to_owned(), format!("{:.0}", player.1)));
    }
    
    result
}

pub fn generate_file(players: &HashMap<String, f32>) {
    let mut text = fs::read_to_string("./base.html").expect("Unable to read file");
    let top_ten = top_players(players);
    for (i, player) in top_ten.iter().enumerate() {
        text = text.replace(&format!("!!!player-{}!!!", i + 1), &player.0);
        text = text.replace(&format!("!!!score-{}!!!", i + 1), &player.1);
    }
    fs::write("./index.html", text).expect("Unable to write file");
}