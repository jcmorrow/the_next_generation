extern crate rand;

use player::Player;
use trade_row::TradeRow;

mod card;
mod player;
mod trade_row;

fn main() {
    let mut player_1 = Player::new("Cameron");
    let mut trade_row = TradeRow::new();
    player_1.take_turn(&mut trade_row);
    player_1.take_turn(&mut trade_row);
    player_1.take_turn(&mut trade_row);
    player_1.take_turn(&mut trade_row);
}
