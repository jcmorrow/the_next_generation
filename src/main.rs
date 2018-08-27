extern crate rand;

use player::Player;

mod card;
mod player;
mod trade_row;

fn main() {
    let mut player_1 = Player::new("Cameron");
    player_1.take_turn();
}
