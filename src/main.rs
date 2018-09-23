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
    let mut trade_row = TradeRow::new();
    let mut player_1 = Player::new("Cameron");
    let mut player_2 = Player::new("Josh");
    let mut players: Vec<Player> = Vec::new();

    players.push(player_1);
    players.push(player_2);

    let mut events: Vec<Event> = Vec::new();
    let mut player_refs: Vec<&Player> = Vec::new();
    let mut turn_count = 0;

    while players.iter().all(|ref p| p.authority > 0)
    {
        let current_player_index = turn_count % players.len();
        let mut current_players = Vec::new();
        let mut opponents = Vec::new();
        let current_player_ref = &players[current_player_index];

        for (i, player) in players.iter_mut().enumerate() {
            if current_player_index == i {
                current_players.push(player);
            } else {
                opponents.push(player);
            }
        }

        let mut current_player = &mut current_players[0];

        print!("Turn {}\n", turn_count);
        print!("{:#}", current_player);


        current_player.begin_turn();
        loop {
            let choice = current_player.make_choice(&trade_row, opponents.as_mut_slice());
            match choice {
                Choice::EndTurn => break,
                _ => {
                    let event = choice.choose(&mut current_player,
                                              opponents.as_mut_slice(),
                                              &mut trade_row,
                                              current_player_ref);
                    // events.push(event);
                }
            };
        }
        current_player.end_turn();


        turn_count = turn_count + 1;
        print!("\n{:#}", trade_row);
    }


    struct Players {
        next_opponent_index: isize,
        current_player_index: isize,
        players: Vec<Player>
    }

    impl Players {
        fn current_and_opponents(&mut self, current: usize) -> (&mut [Player], &mut [Player]) {
            let len = self.players.len();
            let ptr = self.players.as_mut_ptr();
            assert!(current <= len);
            unsafe {
                (std::slice::from_raw_parts_mut(ptr.offset(current as isize), 1),
                std::slice::from_raw_parts_mut(ptr, current).iter().chain(
                    std::slice::from_raw_parts_mut(
                        ptr.offset(current as isize + 1),
                        len - current + 1,
                    )
                ))
            }
        }
    }

    impl Iterator for Players {
        type Item = Player;

        fn next(&mut self) -> Option<Player> {
            if self.current_player_index == self.players.len() {
                self.next_opponent_index = 0;
            } else {
                self.next_opponent_index = self.current_player_index + 1;
            }
            Some(self.players[self.next_opponent_index])
        }
    }
}
