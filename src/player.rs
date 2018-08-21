use card::Card;
use card::ship::Scout;
use card::ship::Viper;

use card::targetable::Targetable;

use std::fmt;

const STARTING_AUTHORITY: i32 = 50;

pub struct Player {
    pub authority: i32,
    pub bases: Vec<Box<Card>>,
    pub combat: i32,
    pub discard: Vec<Box<Card>>,
    pub deck: Vec<Box<Card>>,
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

    pub fn play(&mut self) {
        let card = self.deck.pop().expect("EMPTY DECK!");
        card.play(self);
        self.discard.push(card);
    }
}

impl Targetable for Player {
    fn is_targetable(&self) -> bool {
        self.bases.len() <= 0
    }

    fn receive_combat(&self, mut player: Player, combat: i32) -> Player {
        if player.is_targetable() {
            player.authority -= combat;
        }
        player
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\n", self.name).unwrap();
        write!(f, "Authority: {}\n", self.authority);
        write!(f, "Trade: {}\n", self.trade);
        write!(f, "Combat: {}\n", self.combat);
        write!(f, "Name: {}\n", self.name).unwrap();
        write!(f, "Bases:\n").unwrap();
        for base in self.bases.iter() {
            write!(f, " {}", base);
        }
        // write!(f, "Deck:\n").unwrap();
        // for card in self.deck.iter() {
        //     write!(f, "  {}", card);
        // }
        write!(f, "\n")
    }
}
