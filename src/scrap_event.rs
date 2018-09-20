use card::Card;
use card::CardType;
use card::OutpostType;
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
        match self.card.card_type {
            CardType::Ship => {
                match self.card.ship_type {
                    ShipType::Explorer => self.player.combat += 2,
                    _ => ()
                }
            },
            CardType::Outpost => {
                match self.card.outpost_type {
                    OutpostType::BattleStation => self.player.combat += 5,
                    OutpostType::NoOutpostType => ()
                }
            },
            _ => { println!("Tried to scrap non-scrappable card {}", self.card.name)}
        }

        println!("{} scraps {}", self.player.name, self.card.name);
    }
}
