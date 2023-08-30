use crate::elo::*;

#[derive(Clone, PartialEq)]
pub struct Player {
    name: String,
    ranking: f32,
}
impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            ranking: 1000f32
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct FFAEntrant {
    player: Player,
    score: f32,
}
impl FFAEntrant {
    pub fn new(player: Player, score: f32) -> Self {
        FFAEntrant{player, score}
    }
}

// Integration

pub fn free_for_all(mut players: Vec<Player>, entrants: Vec<(&str, f32)>) -> Vec<Player> {
    let mut ffa_indices: Vec<usize> = Vec::new();
    for entrant in &entrants {
        ffa_indices.push(get_player_index(&players, entrant.0))
    }

    let mut entries: Vec<FFAEntrant> = Vec::new();
    for (i, ffa_index) in ffa_indices.iter().enumerate() {
        entries.push(FFAEntrant::new(players[*ffa_index].clone(), entrants[i].1));
    }

    let mut result: Vec<f32> = Vec::new();
    for entry in &entries {
        let mut ranking = entry.player.ranking;
        for other in &entries {
            if entry == other {
                continue;
            }
            ranking = elo(
                ranking,
                other.player.ranking,
                entry.score / 60f32,
                other.score / 60f32
            ).0;
        }
        result.push(ranking);
    }

    for (i, ffa_index) in ffa_indices.iter().enumerate() {
        players[*ffa_index].ranking = result[i];
    }

    players
}

pub fn one_on_one(
    mut players: Vec<Player>,
    player_a_name: &str,
    player_b_name: &str,
    player_a_score: f32,
    player_b_score: f32)
-> Vec<Player> {
    
    let player_a_index = get_player_index(&players, player_a_name);
    let player_b_index = get_player_index(&players, player_b_name);

    let player_a = players[player_a_index].clone();
    let player_b = players[player_b_index].clone();

    let result = elo(
        player_a.ranking,
        player_b.ranking,
        player_a_score,
        player_b_score
    );

    players[player_a_index].ranking = result.0;
    players[player_b_index].ranking = result.1;

    players
}

pub fn display_players(players: &[Player]) {
    let mut sortable = players.to_owned();
    sortable.sort_unstable_by_key(|element| element.ranking as i32);
    sortable.reverse();
    let total_space = longest_player_name(players) + 2;
    let border = "=".repeat(total_space + 16);
    println!("{}", border);
    for (i, player) in sortable.iter().enumerate() {
        let placement = if (i + 1) > 9 { format!("{} ", i + 1) } else { format!(" {} ", i + 1) };
        let my_space = total_space - player.name.len();
        let spacing_before = " ".repeat(my_space / 2);
        let spacing_after = " ".repeat(my_space - (my_space / 2));
        let ranking = if player.ranking > 999f32 { format!(" {:.0}", player.ranking) } else { format!("  {:.0}", player.ranking) };
        println!(" {} - {}{}{} - {}", placement, spacing_before, player.name, spacing_after, ranking);
    }
    println!("{}", border);
}

fn longest_player_name(players: &[Player]) -> usize {
    let mut sortable = players.to_owned();
    sortable.sort_unstable_by_key(|element| element.name.len());
    sortable.reverse();
    sortable[0].name.len()
}

pub fn get_player_index(players: &[Player], name: &str) -> usize {
    for (i, player) in players.iter().enumerate() {
        if player.name == name {
            return i;
        }
    }
    println!("Break on name \"{name}\"");
    usize::MAX
}