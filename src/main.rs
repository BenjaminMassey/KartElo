mod elo;
mod kart;
mod content;

use crate::kart::*;
use crate::content::*;

use std::io;
use std::collections::HashMap;

fn main() {

    let mut players: HashMap<String, f32> = HashMap::new();

    players = load_content(players);

    display_players(&players);

    io::stdin().read_line(&mut String::new()).unwrap();
}
