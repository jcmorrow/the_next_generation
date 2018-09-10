use card::Card;
use card::ShipType;
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
        match self.card.ship_type {
            ShipType::Explorer => { self.player.combat += 2;},
            _ => { println!("Tried to scrap non-scrappable card {}", self.card.name)}
        }
        println!("{} scraps {}", self.player.name, self.card.name);
    }
}
