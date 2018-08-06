use card::Card;
use card::ship::Scout;

use std::fmt;

const STARTING_AUTHORITY: i32 = 50;

pub struct Player {
    pub name: String,
    pub authority: i32,
    pub trade:  i32,
    pub combat: i32,
    pub discard: Vec<Box<Card>>,
    pub deck: Vec<Box<Card>>,
    pub played: Vec<Box<Card>>,
}

impl Player {
    pub fn new(name: &str) -> Player {
        let mut player = Player{
            name: name.to_string(),
            authority: STARTING_AUTHORITY,
            trade: 0,
            combat: 0,
            discard: Vec::new(),
            deck: Vec::new(),
            played: Vec::new(),
        };

        for _n in 0..8 {
            player.deck.push(Box::new(Scout::new()));
        }

        return player;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\n", self.name).unwrap();
        write!(f, "Trade: {}\n", self.trade)
        // TODO: fix deck logging
        // write!(f, "Name: {}\n", self.name).unwrap();
        // write!(f, "Deck:\n").unwrap();
        // for card in self.deck.iter() {
        //     write!(f, "  {}", card).unwrap();
        // }
        // write!(f, "\n")
    }
}
