use card::Card;
use card::CardType;
use card::Faction;
use card::OutpostType;
use card::ShipType;
use card::ship;
use choice::Choice;
use choice::Ability;
use effect::Effect;
use strategy::RandomStrategy;
use strategy::HeuristicStrategy;
use trade_row::TradeRow;

use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fmt;

const HAND_SIZE: usize = 5;
const STARTING_AUTHORITY: i32 = 50;

#[derive(Debug)]
pub struct Player {
    pub name: String,

    pub abilities: HashMap<Ability, usize>,
    pub choices: Vec<Choice>,
    pub effects: Vec<Effect>,
    pub turn_start_choices: Vec<Choice>,

    pub authority: i32,
    pub combat: i32,
    pub trade:  i32,

    pub deck: Vec<Card>,
    pub discard: Vec<Card>,
    pub hand: Vec<Card>,
    pub in_play: Vec<Card>,
    pub scrapped: Vec<Card>,

    pub blobs_played_this_turn: usize,
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
            abilities: HashMap::new(),
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
            scrapped: Vec::new(),
            trade: 0,
            turn_start_choices: Vec::new()
        };

        player.reset_abilities();

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

    pub fn ships_to_copy(&self) -> Option<usize> {
        let mut any_ships = false;
        let mut highest_cost = 0;
        let mut highest_cost_index: usize = 0;
        for (i, card) in self.in_play.iter().enumerate() {
            if card.card_type == CardType::Ship && card.ship_type != ShipType::StealthNeedle {
                any_ships = true;
                if card.cost >= highest_cost {
                    highest_cost = card.cost;
                    highest_cost_index = i;
                }
            }
        }

        match any_ships {
            false => None,
            true => Some(highest_cost_index)
        }
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

    fn index_of_most_expensive_card(&self,
                                    cards: &Vec<Card>) -> Option<usize> {
        let mut highest_cost = 0;
        let mut highest_cost_index = 0;
        for (index, card) in cards.iter().enumerate() {
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

    fn index_buy_ship_from_trade_row(&self, trade_row: &TradeRow) -> Option<usize> {
        let mut buy_anything = false;
        let mut highest_cost = 0;
        let mut highest_cost_index = 0;
        for (index, card) in trade_row.face_up.iter().enumerate() {
            if card.cost >= highest_cost &&
                self.trade >= card.cost &&
                card.card_type == CardType::Ship {
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

        match self.turn_start_choices.pop() {
            Some(c) => return c,
            None => self.make_strategy_choice(trade_row, opponents),
        }
    }

    pub fn gain_ability(&mut self, ability: Ability) {
        // println!("{} gains the ability: {:?}", self.name, ability);
        let ability_count = self.abilities.entry(ability).or_insert(0);
        *ability_count += 1;
    }


    pub fn reset_abilities(&mut self) {
        self.abilities.clear();
        self.gain_ability(Ability::Attack);
        self.gain_ability(Ability::Buy);
        self.gain_ability(Ability::EndTurn);
        self.gain_ability(Ability::Play);
        self.gain_ability(Ability::ScrapSelf);
    }

    pub fn lose_ability(&mut self, ability: Ability) {
        // println!("{:?}", self.abilities);
        // println!("{} loses the ability: {:?}", self.name, ability);
        let ability_count = self.abilities.entry(ability).or_insert(0);
        *ability_count -= 1;
    }

    pub fn make_strategy_choice(&mut self,
                                trade_row: &TradeRow,
                                opponents: &[&mut Player]) -> Choice {
        self.choices.clear();

        for (ability, count) in &self.abilities {
            if *count > 0 {
                let choices = ability.expand(self, opponents, trade_row);
                self.choices.extend(choices);
            }
        }

        let choice_index = HeuristicStrategy::choose(self, trade_row, opponents);
        let choice = self.choices.remove(choice_index);

        choice
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

        self.reset_abilities();

        let mut abilities_to_gain: Vec<Ability> = Vec::new();
        for base in &self.in_play {
            for ability in &base.abilities {
                abilities_to_gain.push(ability.clone());
            }
        }
        for ability in abilities_to_gain {
            self.gain_ability(ability);
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
            let mut card = self.in_play.remove(*i);

            match card.ship_type {
                ShipType::StealthNeedle => {
                    self.discard.push(ship::stealth_needle())
                },
                _ => self.discard.push(card)
            }
        }
        self.discard.extend(self.hand.drain(0..));
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
        writeln!(f, "Abilities: {:?}", self.abilities).unwrap();

        for card in self.in_play.iter() {
            write!(f, "In play: {}", card).unwrap();
        }
        writeln!(f, "===========").unwrap();
        for card in self.hand.iter() {
            write!(f, "Hand:  {}", card).unwrap();
        }
        writeln!(f, "===========").unwrap();
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
