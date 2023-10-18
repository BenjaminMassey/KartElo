use crate::elo::*;

use std::collections::HashMap;

use ordered_float::NotNan;

// Integration

// Simple Multiplayer Elo http://www.tckerrigan.com/Misc/Multiplayer_Elo/
pub fn free_for_all(players: &mut HashMap<String, f32>, mut entrants: Vec<(&str, f32)>) {
    entrants.sort_by_key(|e| NotNan::new(e.1).unwrap());
    for i in 1..entrants.len() {
        let p1 = &entrants[i - 1];
        let p2 = &entrants[i];
        let r1 = players[p1.0];
        let r2 = players[p2.0];
        let d1 = if p1.1 == p2.1 { 0.5 } else { 0.0 };
        let d2 = if p1.1 == p2.1 { 0.5 } else { 1.0 };
        let (s1, s2) = elo(r1, r2, d1, d2);
        *players.get_mut(p1.0).unwrap() = s1;
        *players.get_mut(p2.0).unwrap() = s2;
    }
}

pub fn display_players(players: &HashMap<String, f32>) {
    let mut sortable: Vec<(&String, &f32)> = players.iter().collect();
    sortable.sort_by(|a, b| (*b.1 as i32).cmp(&(*a.1 as i32)));
    let total_space = longest_player_name(players) + 2;
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
