use crate::elo::*;

use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct FFAEntrant {
    player: String,
    score: f32,
}
impl FFAEntrant {
    pub fn new(player: &str, score: f32) -> Self {
        FFAEntrant{player: player.to_owned(), score}
    }
}

// Integration

pub fn free_for_all(mut players: HashMap<String, f32>, entrants: Vec<(&str, f32)>) -> HashMap<String, f32> {
    let mut entries: Vec<FFAEntrant> = Vec::new();
    for entrant in &entrants {
        entries.push(FFAEntrant::new(entrant.0, entrant.1));
    }

    let mut result: HashMap<String, f32> = HashMap::new();
    for entry in &entries {
        let mut ranking = players[&entry.player];
        for other in &entries {
            if entry == other {
                continue;
            }
            ranking = elo(
                ranking,
                players[&other.player],
                entry.score / 60f32,
                other.score / 60f32
            ).0;
        }
        result.insert(entry.player.clone(), ranking);
    }

    for res in &result {
        players.insert(res.0.to_owned(), *res.1);
    }

    players
}

pub fn one_on_one(
    mut players: HashMap<String, f32>,
    player_a_name: &str,
    player_b_name: &str,
    player_a_score: f32,
    player_b_score: f32)
-> HashMap<String, f32> {

    let player_a_ranking = players[player_a_name];
    let player_b_ranking = players[player_b_name];

    let result = elo(
        player_a_ranking,
        player_b_ranking,
        player_a_score,
        player_b_score
    );

    players.insert(player_a_name.to_owned(), result.0);
    players.insert(player_b_name.to_owned(), result.1);

    players
}

pub fn display_players(players: &HashMap<String, f32>) {
    let mut sortable: Vec<(&String, &f32)> = players.iter().collect();
    sortable.sort_by(|a, b| (*b.1 as i32).cmp(&(*a.1 as i32)));
    let total_space = longest_player_name(&players) + 2;
    let border = "=".repeat(total_space + 16);
    println!("{}", border);
    for (i, player) in sortable.iter().enumerate() {
        let placement = if (i + 1) > 9 { format!("{} ", i + 1) } else { format!(" {} ", i + 1) };
        let my_space: usize = total_space - player.0.len();
        let spacing_before = " ".repeat(my_space / 2);
        let spacing_after = " ".repeat(my_space - (my_space / 2));
        let ranking = if player.1 > &999f32 { format!(" {:.0}", player.1) } else { format!("  {:.0}", player.1) };
        println!(" {} - {}{}{} - {}", placement, spacing_before, player.0, spacing_after, ranking);
    }
    println!("{}", border);
}

fn longest_player_name(players: &HashMap<String, f32>) -> usize {
    
    let mut sortable: Vec<(&String, &f32)> = players.iter().collect();
    sortable.sort_unstable_by_key(|element| element.0.len());
    sortable.reverse();

    sortable[0].0.len()
}