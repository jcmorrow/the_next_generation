use card::Card;

const STARTING_AUTHORITY: i32 = 50;

#[derive(Debug)]
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

        for _n in 1..8 {
            player.deck.push(Card::scout());
        }

        for _n in 1..2 {
            player.deck.push(Card::viper());
        }

        return player;
    }
}
