use card::Card;
use player::Player;

pub struct PlayEvent<'a> {
    player: &'a mut Player,
    card: &'a Card,
}

impl<'a> PlayEvent<'a> {
    pub fn new(card: &'a Card, player: &'a mut Player) -> PlayEvent<'a> {
        PlayEvent {
            card: card,
            player: player,
        }
    }

    pub fn play(&mut self)  {
        self.player.trade += self.card.trade;
        self.player.combat += self.card.combat;
        print!("{} plays {}\n", self.player.name, self.card);

        // We can do special abilities this way
        // match self.card.ship_type {
        //     ShipType::Scout => { self.player.trade += 1; }
        // }
    }
}
