use rand::prelude::*;
use card::Card;
use card::ship::Scout;
use card::ship::Viper;

use card::targetable::Targetable;

use std::fmt;

const STARTING_AUTHORITY: i32 = 50;
const HAND_SIZE: usize = 5;

pub struct Player {
    pub authority: i32,
    pub bases: Vec<Box<Card>>,
    pub combat: i32,
    pub discard: Vec<Box<Card>>,
    pub deck: Vec<Box<Card>>,
    pub hand: Vec<Box<Card>>,
    pub in_play: Vec<Box<Card>>,
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
            name: name.to_string(),
            trade: 0,
        };

        for _n in 0..8 {
            player.deck.push(Box::new(Scout::new()));
        }

        for _n in 0..2 {
            player.deck.push(Box::new(Viper::new()));
        }

        return player;
    }

    pub fn take_turn(&mut self, opponents: &mut [&mut Player]) {
        for _i in 0..HAND_SIZE {
            self.draw();
        }

        while self.hand.len() > 0 {
            let card_to_play = self.hand.pop().unwrap();
            card_to_play.play(self);
            self.discard.push(card_to_play);
        }

        self.attack(opponents);

        self.deck.extend(self.discard.drain(0..));
    }

    pub fn draw(&mut self) {
        let card = self.deck.pop().expect("EMPTY DECK!");
        self.hand.push(card);
    }

    pub fn attack(&mut self, opponents: &mut [&mut Player]) {
        let mut rng = thread_rng();
        let target = &mut opponents[rng.gen_range(0, opponents.len())];
        target.receive_combat(self.combat);
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
        write!(f, "Bases:\n").unwrap();
        for base in self.bases.iter() {
            write!(f, " {}", base).unwrap();
        }
        // write!(f, "Deck:\n").unwrap();
        // for card in self.deck.iter() {
        //     write!(f, "  {}", card);
        // }
        write!(f, "\n")
    }
}
