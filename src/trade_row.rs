use card::Card;
use card::base;
use card::outpost;
use card::ship;

use std::fmt;
use rand::{thread_rng, Rng};

pub struct TradeRow {
    pub deck: Vec<Card>,
    pub face_up: Vec<Card>,
    pub scrapped: Vec<Card>
}

impl TradeRow {
    pub fn buy(&mut self, index: usize) -> (Card) {
        match index {
            5 => self.face_up.insert(5, ship::explorer()),
            i => {
                match self.deck.pop() {
                    Some(card) => self.face_up.insert(i, card),
                    None => (),
                }
            }
        }
        self.face_up.remove(index)
    }

    pub fn scrap(&mut self, index: usize) {
        // Cannot scrap Explorer from trade row
        if index != 0 && index <= self.face_up.len() - 1 {
            let scrapped_card = self.face_up.remove(index);
            println!("{} has been scrapped from the Trade Row!", scrapped_card.name);
            self.scrapped.push(scrapped_card);
            match self.deck.pop() {
                Some(card) => self.face_up.insert(index, card),
                None => ()
            }
        }
    }

    pub fn new() -> TradeRow {
        let mut trade_row = TradeRow{
            deck: Vec::new(),
            face_up: Vec::new(),
            scrapped: Vec::new()
        };

        for _n in 0..15 { trade_row.deck.push(base::the_hive()); }
        for _n in 0..15 { trade_row.deck.push(outpost::battle_station()); }
        for _n in 0..15 { trade_row.deck.push(ship::battle_blob()); }
        for _n in 0..15 { trade_row.deck.push(ship::battle_pod()); }
        for _n in 0..100 { trade_row.deck.push(ship::blob_carrier()); }

        thread_rng().shuffle(&mut trade_row.deck);

        for _n in 0..5 {
            trade_row.face_up.push(trade_row.deck.pop().unwrap());
        }
        trade_row.face_up.insert(5, ship::explorer());
        trade_row
    }
}

impl fmt::Display for TradeRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trade Row:\n").unwrap();
        for card in self.face_up.iter() {
            write!(f, "  {}", card).unwrap();
        }
        write!(f, "\n")
    }
}
