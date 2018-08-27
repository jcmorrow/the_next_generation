use card::ship::PlayEvent;
use card::ship::Card;

use card::targetable::Targetable;

use std::fmt;

use rand::{thread_rng, Rng};

const STARTING_AUTHORITY: i32 = 50;
const HAND_SIZE: usize = 5;

pub struct Player {
    pub authority: i32,
    pub bases: Vec<Card>,
    pub combat: i32,
    pub discard: Vec<Card>,
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub in_play: Vec<Card>,
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
            player.deck.push(Card::scout());
        }

        // for _n in 0..2 {
        //     player.deck.push(Box::new(Viper::new()));
        // }

        thread_rng().shuffle(&mut player.deck);

        return player;
    }

    pub fn take_turn(&mut self) {
        self.draw_hand();
        while self.hand.len() > 0 {
            let card_to_play = self.hand.pop().unwrap();
            PlayEvent::new(&card_to_play, self).play();
            self.discard.push(card_to_play);
        }
    }

    // pub fn take_turn(&mut self, opponents: &mut [&mut Player], trade_row: &mut TradeRow) {
    //     self.combat = 0;
    //     self.trade = 0;
    //     self.draw_hand();

    //     while self.hand.len() > 0 {
    //         let card_to_play = self.hand.pop().unwrap();
    //         card_to_play.play(self);
    //         self.discard.push(card_to_play);
    //     }

    //     self.buy(trade_row);
    //     self.attack(opponents);

    //     self.deck.extend(self.discard.drain(0..));
    // }

    // pub fn buy(&mut self, trade_row: &mut TradeRow) {
    //     let mut rng = thread_rng();
    //     let index = rng.gen_range(0, trade_row.face_up.len());
    //     self.deck.push(trade_row.buy(index));
    // }

    pub fn attack(&mut self, opponents: &mut [&mut Player]) {
        let mut rng = thread_rng();
        let target = &mut opponents[rng.gen_range(0, opponents.len())];
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
        write!(f, "Bases:\n").unwrap();
        for base in self.bases.iter() {
            write!(f, " {}", base).unwrap();
        }
        write!(f, "Deck:\n").unwrap();
        for card in self.hand.iter() {
            write!(f, "Hand:  {}", card).unwrap();
        }
        for card in self.deck.iter() {
            write!(f, "Deck:  {}", card).unwrap();
        }
        write!(f, "\n")
    }
}
