extern crate rand;

use choice::Choice;
use effect::Effect;
use player::Player;
use trade_row::TradeRow;

mod choice;
mod card;
mod effect;
mod player;
mod trade_row;

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
        current_player[0].begin_turn();
        print!("{:#}", current_player[0]);
        loop {
            current_player[0].process_effects(&trade_row, opponents);
            let choice = current_player[0].make_choice(&trade_row, opponents);
            match choice {
                Choice::EndTurn => break,
                _ => (choice.choose(current_player[0], opponents, &mut trade_row))
            };
        }
        current_player[0].end_turn();
        turn_count = turn_count + 1;
        print!("\n{:#}", trade_row);
    }
}
