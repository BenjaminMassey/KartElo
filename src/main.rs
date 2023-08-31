mod elo;
mod kart;
mod content;
mod html;

use crate::kart::*;
use crate::content::*;
use crate::html::*;

use std::io;
use std::collections::HashMap;

fn main() {

    let mut players: HashMap<String, f32> = HashMap::new();

    players = load_content(players);

    display_players(&players);

    generate_file(&players);

    io::stdin().read_line(&mut String::new()).unwrap();
}
