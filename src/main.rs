mod elo;
mod kart;
mod content;

use crate::kart::*;
use crate::content::*;

use std::io;

fn main() {

    let mut players: Vec<Player> = Vec::new();

    players = load_content(players);

    display_players(&players);

    io::stdin().read_line(&mut String::new()).unwrap();
}
