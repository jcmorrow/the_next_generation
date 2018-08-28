extern crate rand;

use player::Player;
use play_event::PlayEvent;
use trade_row::TradeRow;

mod card;
mod player;
mod trade_row;
mod play_event;

fn main() {
    let mut player_1 = Player::new("Cameron");
    let mut player_2 = Player::new("Josh");
    let mut trade_row = TradeRow::new();
    let mut players = Vec::new();
    let mut turn_count = 0;

    players.push(&mut player_1);
    players.push(&mut player_2);

    while !players.iter().any(|ref p| p.authority < 1)
    {
        print!("Turn {}\n", turn_count);
        players.rotate_left(1);
        let (current_player, opponents)  = &mut players.split_at_mut(1);
        print!("{:#}", current_player[0]);
        current_player[0].take_turn(opponents, &mut trade_row);
        turn_count = turn_count + 1;
        print!("{:#}", trade_row);
    }
}
