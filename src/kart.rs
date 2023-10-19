use crate::elo::*;

use std::cmp::Reverse;
use std::collections::HashMap;

use ordered_float::NotNan;

// Simple Multiplayer Elo http://www.tckerrigan.com/Misc/Multiplayer_Elo/
pub fn do_match(
    players: &mut HashMap<String, f32>,
    mat: &HashMap<String, f32>,
) {
    let mut entrants: Vec<_> = mat.iter().collect();
    entrants.sort_by_key(|e| NotNan::new(*e.1).unwrap());
    for i in 1..entrants.len() {
        let p1 = &entrants[i - 1];
        let p2 = &entrants[i];
        let r1 = players[p1.0];
        let r2 = players[p2.0];
        let d1: f32;// = if p1.1 == p2.1 { 0.5 } else { 0.0 };
        let d2: f32;// = if p1.1 == p2.1 { 0.5 } else { 1.0 };
        if (0f32..=1f32).contains(&p1.1) && (0f32..=1f32).contains(&p2.1) {
            d1 = *p1.1; d2 = *p2.1;
        } else {
            // Win / Loss Approach (Greater Value)
            /*
            d1 = if p1.1 == p2.1 { 0.5 } else { 0.0 };
            d2 = if p1.1 == p2.1 { 0.5 } else { 1.0 };
            */

            // Percentage of Max Score Approach
            d1 = p1.1 / 60f32;
            d2 = p2.1 / 60f32;

            // Normalize Scores Approach
            /*
            let sum = *p1.1 + *p2.1;
            d1 = p1.1 / sum;
            d2 = p2.1 / sum;
            */
        }
        let (s1, s2) = elo(r1, r2, d1, d2);
        *players.get_mut(p1.0).unwrap() = s1;
        *players.get_mut(p2.0).unwrap() = s2;
    }
}

pub fn display_players(players: &HashMap<String, f32>) {
    let mut sortable: Vec<(&String, &f32)> = players.iter().collect();
    sortable.sort_by_key(|e| Reverse(NotNan::new(*e.1).unwrap()));
    let mut last_placement: usize = 0;
    let mut last_ranking: String = "".to_owned();
    let total_space = longest_player_name(players) + 2;
    let border = "=".repeat(total_space + 16);
    println!("{}", border);
    for (i, player) in sortable.iter().enumerate() {
        let ranking = if player.1 > &999f32 { format!(" {:.0}", player.1) } else { format!("  {:.0}", player.1) };
        let placement: usize;
        if ranking == last_ranking {
            placement = last_placement;
        } else {
            placement = i;
            last_placement = placement;
            last_ranking = ranking.clone();
        }
        let placement_str = if (placement + 1) > 9 { format!("{} ", placement + 1) } else { format!(" {} ", placement + 1) };
        let my_space: usize = total_space - player.0.len();
        let spacing_before = " ".repeat(my_space / 2);
        let spacing_after = " ".repeat(my_space - (my_space / 2));
        println!(" {} - {}{}{} - {}", placement_str, spacing_before, player.0, spacing_after, ranking);
    }
    println!("{}", border);
}

fn longest_player_name(players: &HashMap<String, f32>) -> usize {
    
    let mut sortable: Vec<(&String, &f32)> = players.iter().collect();
    sortable.sort_unstable_by_key(|element| element.0.len());
    sortable.reverse();

    sortable[0].0.len()
}
