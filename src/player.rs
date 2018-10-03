use choice::Choice;
use card::Card;
use card::CardType;
use card::Faction;
use card::OutpostType;
use card::ship;
use effect::Effect;
use trade_row::TradeRow;

use rand::{random, thread_rng, Rng};
use std::fmt;

const HAND_SIZE: usize = 5;
const STARTING_AUTHORITY: i32 = 50;

#[derive(Debug)]
pub struct Player {
    pub authority: i32,
    pub blobs_played_this_turn: usize,
    pub choices: Vec<Choice>,
    pub combat: i32,
    pub deck: Vec<Card>,
    pub discard: Vec<Card>,
    pub effects: Vec<Effect>,
    pub hand: Vec<Card>,
    pub in_play: Vec<Card>,
    pub name: String,
    pub perrenial_choices: Vec<Choice>,
    pub scrapped: Vec<Card>,
    pub trade:  i32,
    pub turn_start_choices: Vec<Choice>,
}

pub enum CardPile {
    Discard,
    Deck,
    Hand,
    InPlay
}

impl Player {
    pub fn new(name: &str) -> Player {
        let mut player = Player {
            authority: STARTING_AUTHORITY,
            blobs_played_this_turn: 0,
            choices: Vec::new(),
            combat: 0,
            discard: Vec::new(),
            deck: Vec::new(),
            effects: Vec::new(),
            hand: Vec::new(),
            in_play: Vec::new(),
            name: name.to_string(),
            perrenial_choices: Vec::new(),
            scrapped: Vec::new(),
            trade: 0,
            turn_start_choices: Vec::new()
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

    pub fn has_ally_in_play(&self, faction: &Faction) -> bool {
        for card in &self.in_play {
            if card.faction == *faction {
                return true
            }
            match card.outpost_type {
                OutpostType::MechWorld => return true,
                _ => ()
            }
        }
        false
    }

    pub fn bases(&self) -> Vec<usize> {
        let mut bases: Vec<usize> = Vec::new();
        for (i, card) in self.in_play.iter().enumerate() {
            if card.card_type == CardType::Outpost || card.card_type == CardType::Base {
                bases.push(i);
            }
        }
        bases
    }

    fn index_attack_opponents(&self, opponents: &[&mut Player]) -> Option<usize> {
        match opponents.len() {
            0 => None,
            _ => {
                match self.combat {
                    0 => None,
                    _ => Some(0),
                }
            }
        }
    }

    fn indices_destroy_base(&self, opponents: &[&mut Player]) -> Option<(usize, usize)> {
        for (index, opponent) in opponents.iter().enumerate() {
            if opponent.bases().len() > 0 {
                return Some((index, 0));
            }
        }
        None
    }

    fn index_discard_opponents(&self, opponents: &[&mut Player]) -> Option<usize> {
        match opponents.len() {
            0 => None,
            _ => Some(0)
        }
    }

    fn index_acquire_from_trade_row(&self,
                                    trade_row: &TradeRow) -> Option<usize> {
        let mut highest_cost = 0;
        let mut highest_cost_index = 0;
        for (index, card) in trade_row.face_up.iter().enumerate() {
            if card.cost >= highest_cost {
                highest_cost = card.cost;
                highest_cost_index = index;
            }
        }
        Some(highest_cost_index)
    }

    fn index_buy_from_trade_row(&self, trade_row: &TradeRow) -> Option<usize> {
        let mut buy_anything = false;
        let mut highest_cost = 0;
        let mut highest_cost_index = 0;
        for (index, card) in trade_row.face_up.iter().enumerate() {
            if card.cost >= highest_cost && self.trade >= card.cost {
                highest_cost = card.cost;
                highest_cost_index = index;
                buy_anything = true;
            }
        }
        match buy_anything {
            true => Some(highest_cost_index),
            false => None,
        }
    }

    fn index_scrap_from_trade_row(&self, trade_row: &TradeRow) -> Option<usize> {
        match trade_row.face_up.len() {
            0 => None,
            _ => Some(0)
        }
    }

    pub fn index_from(&self, card_pile: CardPile) -> Option<usize> {
        let card_pile = match card_pile {
            CardPile::Discard => &self.discard,
            CardPile::Deck => &self.deck,
            CardPile::Hand => &self.hand,
            CardPile::InPlay => &self.in_play
        };

        match card_pile.len() {
            0 => None,
            _ => Some(0)
        }
    }

    fn make_perennial_choice(&mut self,
                             trade_row: &TradeRow,
                             opponents: &[&mut Player]) -> Choice {
        match self.perrenial_choices.pop() {
            Some(c) => c,
            None => {
                match self.choices.pop() {
                    Some(c) => match c {
                        Choice::AcquireFromTradeRow(_) => {
                            match self.index_acquire_from_trade_row(trade_row) {
                                Some(i) => Choice::AcquireFromTradeRow(i),
                                None => Choice::Decline,
                            }
                        },
                        Choice::DestroyBase(_, _) => {
                            match self.indices_destroy_base(opponents) {
                                Some(opponent_base) => Choice::DestroyBase(
                                    // opponnent index
                                    opponent_base.0,
                                    // base index
                                    opponent_base.1,
                                ),
                                None => Choice::Decline,
                            }
                        },
                        Choice::BlobDraw(_) => {
                            match self.blobs_played_this_turn {
                                0 => Choice::Decline,
                                n => Choice::BlobDraw(n),
                            }
                        },
                        Choice::ScrapDiscard(_) => {
                            match self.index_from(CardPile::Discard) {
                                Some(i) => Choice::ScrapDiscard(i),
                                None => Choice::Decline
                            }
                        },
                        Choice::ScrapHand(_) => {
                            match self.index_from(CardPile::Hand) {
                                Some(i) => Choice::ScrapHand(i),
                                None => Choice::Decline
                            }
                        },
                        Choice::Or(a, b, _) => {
                            // Stupid robot choices
                            Choice::Or(a, b, random())
                        },
                        Choice::AndOr(a, b, _, _) => {
                            // Stupid robot choices
                            Choice::AndOr(a, b, random(), random())
                        },
                        Choice::ScrapFromTradeRow(_) => {
                            match self.index_scrap_from_trade_row(trade_row) {
                                Some(i) => Choice::ScrapFromTradeRow(i),
                                None => Choice::Decline
                            }
                        },
                        c => c
                    },
                    None => Choice::EndTurn
                }
            }
        }
    }

    pub fn process_effects(&mut self,
                           trade_row: &TradeRow,
                           opponents: &mut [&mut Player]) {
        let mut effects = Vec::new();
        effects.extend(self.effects.drain(0..));

        for effect in &effects {
            match effect {
                Effect::DiscardAttack(_) => {
                    let e = match self.index_discard_opponents(opponents) {
                        Some(i) => Effect::DiscardAttack(i),
                        None => Effect::Empty
                    };
                    e.process(self, opponents, trade_row)
                },
                e => e.process(self, opponents, trade_row)
            }
        }
    }


    pub fn make_choice(&mut self,
                       trade_row: &TradeRow,
                       opponents: &[&mut Player]) -> Choice {
        if self.turn_start_choices.len() > 0 {
            return self.turn_start_choices.pop().unwrap()
        }

        self.perrenial_choices.clear();
        match self.index_attack_opponents(opponents) {
            Some(i) => self.perrenial_choices.push(Choice::Attack(i)),
            None => ()
        }

        match self.index_buy_from_trade_row(trade_row) {
            Some(i) => self.perrenial_choices.push(Choice::Buy(i)),
            None => ()
        }

        match self.index_from(CardPile::Hand) {
            Some(i) => self.perrenial_choices.push(Choice::Play(i)),
            None => ()
        }

        for (index, card) in self.in_play.iter().enumerate() {
            if card.scrap_abilities.len() > 0 || card.scrap_effects.len() > 0 {
                self.perrenial_choices.push(Choice::ScrapSelf(index));
            }
        }

        match self.turn_start_choices.pop() {
            Some(c) => match c {
                Choice::DiscardCard(_) => {
                    match self.index_from(CardPile::Hand) {
                        Some(i) => Choice::DiscardCard(i),
                        None => self.make_perennial_choice(trade_row, opponents)
                    }
                },
                // Note: this should never happen, since only DiscardCard choices are at turn start
                c => c
            },
            None => self.make_perennial_choice(trade_row, opponents)
        }
    }

    pub fn draw(&mut self) {
        match self.deck.pop() {
            Some(c) => self.hand.push(c),
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

    pub fn begin_turn(&mut self) {
        self.combat = 0;
        self.trade = 0;

        let mut num_to_draw = HAND_SIZE;
        let total_cards = self.deck.len() + self.discard.len();

        if total_cards < HAND_SIZE {
            num_to_draw = total_cards;
        }

        for _i in 0..num_to_draw {
            self.draw();
        }

        for base in &self.in_play {
            self.choices.extend(base.abilities.clone());
        }
    }

    pub fn end_turn(&mut self) {
        let mut to_discard: Vec<usize> = Vec::new();
        for (i, card) in self.in_play.iter().enumerate() {
            if card.card_type == CardType::Ship {
                to_discard.push(i);
            }
        }
        for i in to_discard.iter().rev() {
            self.discard.push(self.in_play.remove(*i));
        }
        for card in &mut self.discard { card.has_used_ally_ability = false; }
        for card in &mut self.deck { card.has_used_ally_ability = false; }
        for card in &mut self.hand { card.has_used_ally_ability = false; }
        for card in &mut self.in_play { card.has_used_ally_ability = false; }
        for card in &mut self.scrapped { card.has_used_ally_ability = false; }
        self.turn_start_choices.clear();
        self.blobs_played_this_turn = 0;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\n", self.name).unwrap();
        write!(f, "Authority: {}\n", self.authority).unwrap();
        write!(f, "Trade: {}\n", self.trade).unwrap();
        write!(f, "Combat: {}\n", self.combat).unwrap();
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
