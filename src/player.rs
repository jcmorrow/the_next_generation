use PlayEvent;
use AttackEvent;
use card::Card;
use card::CardType;
use card::ShipType;
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

        while self.hand.len() > 0 {
            let card_to_play = self.hand.pop().unwrap();
            PlayEvent::new(&card_to_play, self).play();

            match card_to_play.card_type {
                CardType::Ship => { (self.in_play.push(card_to_play)) },
                CardType::Outpost => { self.bases.push(card_to_play) },
                _ => { panic!("Oh noes!") }
            }
        }

        self.scrap_played_cards();
        self.buy(trade_row);
        self.attack(opponents);

        self.discard.extend(self.in_play.drain(0..));
    }

    pub fn scrap_played_cards(&mut self) {
        for iter in self.in_play.iter().enumerate() {
            if iter.1.scrappable {
                match iter.1.ship_type {
                    ShipType::Explorer => { self.combat += 2;},
                    _ => { print!("Tried to scrap non-scrappable card {}\n", iter.1.name)}
                }
                // TODO: move card from in_play to scrapped
                print!("{} scraps {}", self.name, iter.1.name);
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
        self.hand.push(self.deck.pop().expect("Empty deck"));
    }

    pub fn draw_hand(&mut self) {
        let mut num_to_draw = HAND_SIZE;
        let total_cards = self.deck.len() + self.discard.len();

        if total_cards < HAND_SIZE {
            num_to_draw = total_cards;
        }

        for _i in 0..num_to_draw {
            if self.deck.len() < 1 {
                self.deck.extend(self.discard.drain(0..));
                thread_rng().shuffle(&mut self.deck);
            }
            self.draw();
        }
    }
}

impl Targetable for Player {
    fn is_targetable(&self) -> bool {
        self.bases.len() <= 0
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
