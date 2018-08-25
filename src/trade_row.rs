use std::slice;
use card::Card;
use card::ship::Scout;
use card::ship::Viper;
use card::ship::Explorer;

use std::fmt;

pub struct TradeRow {
    pub deck: Vec<Box<Card>>,
    pub face_up: Vec<Box<Card>>,
}

impl TradeRow {
    pub fn buy(&mut self, index: usize) -> Box<Card> {
        if index == 0 {
            self.face_up.insert(0, Box::new(Explorer::new()));
        } else {
            match self.deck.pop() {
                Some(card) => self.face_up.insert(5, card),
                None => (),
            }
        }
        self.face_up.remove(index)
    }

    pub fn new() -> TradeRow {
        let mut trade_row = TradeRow{
            deck: Vec::new(),
            face_up: Vec::new(),
        };

        for _n in 0..50 {
            trade_row.deck.push(Box::new(Viper::new()));
        }
        trade_row.face_up.push(Box::new(Explorer::new()));
        for _n in 0..5 {
            trade_row.face_up.push(trade_row.deck.pop().unwrap());
        }
        trade_row
    }
}

impl fmt::Display for TradeRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trade Row:\n").unwrap();
        for card in self.face_up.iter() {
            write!(f, "  {}", card);
        }
        write!(f, "\n")
    }
}
