use AllyAbilityEvent;
use AttackEvent;
use PlayEvent;
use ScrapEvent;
use card::Card;
use card::CardType;
use card::Faction;
use trade_row::TradeRow;

use card::targetable::Targetable;
use std::fmt;
use rand::{thread_rng, Rng};

const STARTING_AUTHORITY: i32 = 50;
const HAND_SIZE: usize = 5;

pub struct Player {
    pub authority: i32,
    pub bases: Vec<Card>,
    pub combat: i32,
    pub deck: Vec<Card>,
    pub discard: Vec<Card>,
    pub hand: Vec<Card>,
    pub in_play: Vec<Card>,
    pub scrapped: Vec<Card>,
    pub name: String,
    pub trade:  i32,
}

impl Player {
    pub fn new(name: &str) -> Player {
        let mut player = Player{
            authority: STARTING_AUTHORITY,
            bases: Vec::new(),
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
            player.deck.push(Card::scout());
        }

        for _n in 0..2 {
            player.deck.push(Card::viper());
        }

        thread_rng().shuffle(&mut player.deck);

        return player;
    }

    pub fn take_turn(&mut self,
                     opponents: &mut [&mut Player],
                     trade_row: &mut TradeRow) {
        self.combat = 0;
        self.trade = 0;

        self.draw_hand();

        self.trigger_base_ally_abilities();

        while self.hand.len() > 0 {
            let card_to_play = self.hand.pop().unwrap();
            PlayEvent::new(&card_to_play, self).play();
            AllyAbilityEvent::new(&card_to_play, self).trigger_ability();

            match card_to_play.card_type {
                CardType::Ship => { (self.in_play.push(card_to_play)) },
                CardType::Outpost => { self.bases.push(card_to_play) },
                CardType::Base => { self.bases.push(card_to_play) },
                _ => { panic!("I don't know how to play this card type!") }
            }
        }

        self.scrap_played_cards();
        self.scrap_bases();
        self.buy(trade_row);
        self.attack(opponents);
    }

    pub fn scrap_bases(&mut self) {
        let mut indices: Vec<usize> = Vec::new();

        for (i, base) in self.bases.iter().enumerate() {
            if base.scrappable {
                indices.push(i);
            }
        }

        for index in indices.iter().rev() {
            let base = self.bases.remove(*index);
            ScrapEvent::new(&base, self).scrap();
            self.scrapped.push(base);
        }
    }

    pub fn scrap_played_cards(&mut self) {
        while self.in_play.len() > 0 {
            let card_in_play = self.in_play.pop().unwrap();

            if card_in_play.scrappable {
                ScrapEvent::new(&card_in_play, self).scrap();
                self.scrapped.push(card_in_play);
            } else {
                self.discard.push(card_in_play);
            }
        }
    }

    pub fn buy(&mut self, trade_row: &mut TradeRow) {
         while self.trade > 0 {
            let mut options = trade_row.face_up.clone();
            options.sort_unstable_by(|a, b| b.cost.cmp(&a.cost));
            let card_to_buy: &Card = match options.iter()
                .find(|card| card.cost < self.trade) {
                Some(card) => card,
                None => return (),
            };
            let index_of_card_to_buy = match trade_row.face_up.iter()
                .enumerate()
                .find(|(_index, card)| card.name == card_to_buy.name) {
                Some((index, _card)) => index,
                None => panic!("Couldn't find card in trade row!"),
            };
            let card = trade_row.buy(index_of_card_to_buy);
            self.trade -= card.cost;
            self.discard.push(card);
         }
    }

    pub fn attack(&mut self, opponents: &mut [&mut Player]) {
        let mut rng = thread_rng();
        let target = &mut opponents[rng.gen_range(0, opponents.len())];
        AttackEvent::new(self.combat, self, target).log();
        target.receive_combat(self.combat);
    }

    pub fn draw(&mut self) {
        match self.deck.pop() {
            Some(x) => self.hand.push(x),
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

    pub fn has_factions_in_play(&self, faction: &Faction) -> bool {
        let mut count = 0;

        for card in &[&self.bases[..], &self.in_play[..]].concat() {
            if card.faction == *faction {
                count += 1;
            }
            if count > 1 {
                return true
            }
        }
        false
    }

    pub fn trigger_base_ally_abilities(&mut self) {
        let bases = self.bases.clone();
        for base in bases {
            AllyAbilityEvent::new(&base, self).trigger_ability();
        }
    }
}

impl Targetable for Player {
    fn is_targetable(&self) -> bool {
        !self.bases.iter().any(|ref base| base.card_type == CardType::Outpost)
    }

    fn receive_combat(&mut self, combat: i32) {
        if self.is_targetable() {
            self.authority -= combat;
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
use player::Player;
use card::Card;
use card::Faction;

    #[test]
    fn has_no_factions_in_play() {
        let mut player: Player = Player::new("Testy");
        player.in_play.push(Card::battle_blob());
        assert!(!player.has_factions_in_play(&Faction::Blob));
    }

    #[test]
    fn has_factions_in_play() {
        let mut player: Player = Player::new("Testy");
        player.in_play.push(Card::battle_blob());
        player.in_play.push(Card::battle_blob());
        print!("{}", player);
        assert!(player.has_factions_in_play(&Faction::Blob));
    }

    #[test]
    fn battle_blobs_draw() {
        let mut player: Player = Player::new("Testy");

        assert_eq!(player.hand.len(), 0);

        player.bases.push(Card::the_hive());
        player.trigger_base_ally_abilities();

        assert_eq!(player.hand.len(), 0);

        player.bases.push(Card::the_hive());
        player.trigger_base_ally_abilities();

        assert_eq!(player.hand.len(), 2);
    }

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
