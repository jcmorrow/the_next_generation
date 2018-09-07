use card::Card;

use std::fmt;

pub struct TradeRow {
    pub deck: Vec<Card>,
    pub face_up: Vec<Card>,
}

impl TradeRow {
    pub fn buy(&mut self, index: usize) -> (Card) {
        if index == 0 {
            self.face_up.insert(0, Card::explorer());
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

        for _n in 0..25 {
            trade_row.deck.push(Card::battle_blob());
        }
        for _n in 0..25 {
            trade_row.deck.push(Card::battle_station());
        }
        trade_row.face_up.push(Card::explorer());
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
            write!(f, "  {}", card).unwrap();
        }
        write!(f, "\n")
    }
}
