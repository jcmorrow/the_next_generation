// use AllyAbilityEvent;
// use AttackEvent;
// use PlayEvent;
// use ScrapEvent;

use choice::Choice;
use card::Card;
use card::ship;
use trade_row::TradeRow;

use rand::{thread_rng, Rng};
use std::fmt;

const HAND_SIZE: usize = 5;
const STARTING_AUTHORITY: i32 = 50;

#[derive(Debug)]
pub struct Player {
    pub authority: i32,
    pub bases: Vec<Card>,
    pub choices: Vec<Choice>,
    pub combat: i32,
    pub deck: Vec<Card>,
    pub discard: Vec<Card>,
    pub hand: Vec<Card>,
    pub in_play: Vec<Card>,
    pub name: String,
    pub scrapped: Vec<Card>,
    pub trade:  i32,
}

impl Player {
    pub fn new(name: &str) -> Player {
        let mut player = Player{
            authority: STARTING_AUTHORITY,
            bases: Vec::new(),
            choices: Vec::new(),
            combat: 0,
            discard: Vec::new(),
            deck: Vec::new(),
            hand: Vec::new(),
            in_play: Vec::new(),
            scrapped: Vec::new(),
            name: name.to_string(),
            trade: 0,
        };

        for _n in 0..8 {
            player.deck.push(ship::scout());
        }

        for _n in 0..2 {
            player.deck.push(ship::viper());
        }

        thread_rng().shuffle(&mut player.deck);

        return player;
    }

    fn index_from_trade_row(&self, trade_row: &TradeRow) -> usize {
        let mut highest_cost = 0;
        let mut highest_cost_index = 0;
        for (index, card) in trade_row.face_up.iter().enumerate() {
            if card.cost >= highest_cost {
                highest_cost = card.cost;
                highest_cost_index = index;
            }
        }
        highest_cost_index
    }

    pub fn make_choice(&mut self, trade_row: &TradeRow) -> Choice {
        return match self.choices.pop() {
            Some(c) => {
                match c {
                    Choice::AcquireFromTradeRow(_) => {
                        Choice::AcquireFromTradeRow(self.index_from_trade_row(trade_row))
                    },
                    _ => c
                }
            },
            None => Choice::EndTurn,
        }
    }

    pub fn draw(&mut self) {
        match self.deck.pop() {
            Some(x) => {
                self.choices.push(Choice::Play(x));
            }
            None => {
                self.deck.extend(self.discard.drain(0..));
                thread_rng().shuffle(&mut self.deck);
                match self.deck.pop() {
                    Some(x) => self.hand.push(x),
                    None => (),
                };
            }
        };
    }

    pub fn draw_hand(&mut self) {
        let mut num_to_draw = HAND_SIZE;
        let total_cards = self.deck.len() + self.discard.len();

        if total_cards < HAND_SIZE {
            num_to_draw = total_cards;
        }

        for _i in 0..num_to_draw {
            self.draw();
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\n", self.name).unwrap();
        write!(f, "Authority: {}\n", self.authority).unwrap();
        write!(f, "Trade: {}\n", self.trade).unwrap();
        write!(f, "Combat: {}\n", self.combat).unwrap();
        for base in self.bases.iter() {
            write!(f, "Base: {}", base).unwrap();
        }
        for card in self.in_play.iter() {
            write!(f, "In play: {}", card).unwrap();
        }
        for card in self.hand.iter() {
            write!(f, "Hand:  {}", card).unwrap();
        }
        for card in self.deck.iter() {
            write!(f, "Deck:  {}", card).unwrap();
        }
        for card in self.discard.iter() {
            write!(f, "Discard:  {}", card).unwrap();
        }
        write!(f, "\n")
    }
}

#[cfg(test)]
mod tests {
use card::Faction;
use card::base;
use card::ship;
use player::Player;

    #[test]
    fn overdraw() {
        let mut player: Player = Player::new("Testy");

        for _ in 0..12 {
            // Attempt to draw 11 times
            player.draw();
        }
        assert_eq!(player.hand.len(), 10);
        assert_eq!(player.deck.len(), 0);
        assert_eq!(player.discard.len(), 0);
    }
}
