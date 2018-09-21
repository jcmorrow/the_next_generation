extern crate rand;

// use ally_ability_event::AllyAbilityEvent;
// use attack_event::AttackEvent;
// use play_event::PlayEvent;
// use scrap_event::ScrapEvent;
use choice::Choice;
use player::Player;
use trade_row::TradeRow;

// mod ally_ability_event;
// mod attack_event;
// mod play_event;
// mod scrap_event;
mod choice;
mod card;
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
        print!("{:#}", current_player[0]);
        loop {
            current_player[0].draw_hand();
            let choice = current_player[0].make_choice();
            println!("{} {}", current_player[0].name, choice);
            match choice {
                Choice::EndTurn => { break; }
                _ => (choice.choose(current_player[0], opponents, &mut trade_row))
            };
        }
        if turn_count >= 1 {
            println!("Game ends");
            break;
        }
        turn_count = turn_count + 1;
        print!("\n{:#}", trade_row);
    }
}
