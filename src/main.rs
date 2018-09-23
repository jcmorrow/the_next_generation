extern crate rand;

use choice::Choice;
use choice::Event;
use player::Player;
use trade_row::TradeRow;

mod choice;
mod card;
mod player;
mod trade_row;

fn main() {
    let player_1 = Player::new("Cameron");
    let player_2 = Player::new("Josh");
    let mut trade_row = TradeRow::new();
    let mut players = vec!(player_1, player_2);

    let mut events: Vec<Event> = Vec::new();
    let mut turn_count = 0;

    while players.iter().all(|ref p| p.authority > 0)
    {
        let mut current_player = players.pop().unwrap();
        print!("Turn {}\n", turn_count);
        current_player.begin_turn();
        print!("{:#}", current_player);
        loop {
            let choice = current_player.make_choice(&trade_row, &players[..]);
            match choice {
                Choice::EndTurn => break,
                _ => {
                    let event = choice.choose(&mut current_player,
                                              &mut players[..],
                                              &mut trade_row);
                    // events.push(event)
                }
            };
        }
        current_player.end_turn();
        turn_count = turn_count + 1;
        print!("\n{:#}", trade_row);
        players.insert(0, current_player);
    }
}
