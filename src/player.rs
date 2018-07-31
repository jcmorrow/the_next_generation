use card::Card;
use std::fmt;

const STARTING_AUTHORITY: i32 = 50;

pub struct Player {
    pub name: String,
    pub authority: i32,
    pub deck: Vec<Card>,
}

impl Player {
    pub fn new(name: &str) -> Player {
        let mut player = Player{
            name: name.to_string(),
            authority: STARTING_AUTHORITY,
            deck: Vec::new(),
        };

        for _n in 0..8 {
            player.deck.push(Card::scout());
        }

        for _n in 0..2 {
            player.deck.push(Card::viper());
        }

        return player;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut deck_string = String::new();
        write!(f, "Name: {}\n", self.name);
        write!(f, "Deck:\n");
        for card in self.deck.iter() {
            write!(f, "  {}", card);
        }
        write!(f, "\n")
    }
}
