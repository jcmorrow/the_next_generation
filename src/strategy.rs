use choice::Choice;
use choice::Ability;
use player::Player;
use trade_row::TradeRow;
use card::ShipType;

use rand::{thread_rng, Rng};

pub struct RandomStrategy {}
pub struct HeuristicStrategy {}

impl RandomStrategy {
    pub fn choose(player: &Player, trade_row: &TradeRow, opponents: &[&mut Player]) -> usize {
        let mut choice = thread_rng().gen_range(0, player.choices.len());
        if player.choices.len() > 1 {
            // don't end turn unless there is nothing else to do
            while player.choices[choice] == Choice::EndTurn {
                choice = thread_rng().gen_range(0, player.choices.len());
            }
        }
        choice
    }
}

impl HeuristicStrategy {
    pub fn choose(player: &Player, trade_row: &TradeRow, opponents: &[&mut Player]) -> usize {
        let ability_priorities = vec!(Ability::Play);
        // try and find the prioritized abilities first, failing that do anything else.
        for ability in ability_priorities {
            for (i, choice) in player.choices.iter().enumerate() {
                if choice.to_ability() == ability {
                    return i
                }
            }
        }

        let buy_choice = HeuristicStrategy::buy_choice(player, trade_row);

        if buy_choice >= 0 {
            return buy_choice as usize;
        }

        let mut choice = thread_rng().gen_range(0, player.choices.len());
        if player.choices.len() > 1 {
            // don't end turn unless there is nothing else to do
            while player.choices[choice] == Choice::EndTurn {
                choice = thread_rng().gen_range(0, player.choices.len());
            }
        }
        choice
    }

    pub fn buy_choice(player: &Player, trade_row: &TradeRow) -> isize {
        let mut average_card_cost = 0;
        let mut card_count = 0;
        for card in &player.deck {
            if card.ship_type != ShipType::Viper &&
               card.ship_type != ShipType::Scout {
                average_card_cost += card.cost;
                card_count += 1;
               }
        }
        for card in &player.discard {
            if card.ship_type != ShipType::Viper &&
               card.ship_type != ShipType::Scout {
                average_card_cost += card.cost;
                card_count += 1;
               }
        }
        for card in &player.in_play {
            if card.ship_type != ShipType::Viper &&
               card.ship_type != ShipType::Scout {
                average_card_cost += card.cost;
                card_count += 1;
               }
        }
        for card in &player.hand {
            if card.ship_type != ShipType::Viper &&
               card.ship_type != ShipType::Scout {
                average_card_cost += card.cost;
                card_count += 1;
               }
        }
        let average_card_cost = if card_count > 0 {
             average_card_cost as f64 / card_count as f64
        } else {
            0 as f64
        };
        println!("Average card cost: {}", average_card_cost);

        let mut best_buy_difference = 0.0;
        let mut best_buy: isize = -1;

        for (i, choice) in player.choices.iter().enumerate() {
            match choice {
                Choice::Buy(n) => {
                    if trade_row.face_up[*n].cost as f64 - average_card_cost > best_buy_difference as f64 {
                        best_buy_difference = trade_row.face_up[*n].cost as f64 - average_card_cost;
                        best_buy = i as isize;
                    }
                }
                _ => ()
            }
        }
        best_buy
    }
}
