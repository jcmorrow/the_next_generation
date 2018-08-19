use card::Card;
use card::ship::Scout;
use card::ship::Viper;

use card::targetable::Targetable;

use std::fmt;

const STARTING_AUTHORITY: i32 = 50;

pub struct Player {
    pub name: String,
    pub authority: i32,
    pub trade:  i32,
    pub combat: i32,
    pub discard: Vec<Box<Card>>,
    pub deck: Vec<Box<Card>>,
    pub in_play: Vec<Box<Card>>
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
            in_play: Vec::new()
        };

        for _n in 0..8 {
            player.deck.push(Box::new(Scout::new()));
        }

        for _n in 0..2 {
            player.deck.push(Box::new(Viper::new()));
        }

        return player;
    }
}

impl Targetable for Player {
    fn is_targetable(&self) -> bool {
        // TODO: player is not targetable when outposts are in play
        true
    }

    fn process_attack(&self, mut player: Player, combat: i32) -> Player {
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
        write!(f, "Deck:\n").unwrap();
        for card in self.deck.iter() {
            write!(f, "  {}", card);
        }
        write!(f, "\n")
    }
}
