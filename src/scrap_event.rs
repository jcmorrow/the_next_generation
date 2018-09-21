use card::Card;
use card::Type;
use card::outpost;
use card::ship;
use player::Player;

pub struct ScrapEvent<'a> {
    player: &'a mut Player,
    card: &'a Card,
}

impl<'a> ScrapEvent<'a> {
    pub fn new(card: &'a Card, player: &'a mut Player) -> ScrapEvent<'a> {
        ScrapEvent {
            card: card,
            player: player,
        }
    }

    pub fn scrap(&mut self)  {
        match self.card.card_type {
            Type::Ship => {
                match self.card.ship_type {
                    ship::Type::Explorer => self.player.combat += 2,
                    _ => ()
                }
            },
            Type::Outpost => {
                match self.card.outpost_type {
                    outpost::Type::BattleStation => self.player.combat += 5,
                    outpost::Type::NoType => ()
                }
            },
            _ => { println!("Tried to scrap non-scrappable card {}", self.card.name)}
        }

        println!("{} scraps {}", self.player.name, self.card.name);
    }
}
