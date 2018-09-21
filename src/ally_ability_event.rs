use card::base;
use card::Card;
use card::Type;
use card::ship;
use player::Player;

pub struct AllyAbilityEvent<'a> {
    player: &'a mut Player,
    card: &'a Card,
}

impl<'a> AllyAbilityEvent<'a> {
    pub fn new(card: &'a Card,
               player: &'a mut Player) -> AllyAbilityEvent<'a> {
        AllyAbilityEvent { card: card, player: player }
    }

    pub fn trigger_ability(&mut self) {

        print!("{} uses the ally ability of {}\n",
               self.player.name,
               self.card.name);

        match self.card.card_type {
            Type::Ship => {
                match self.card.ship_type {
                    ship::Type::BattlePod => self.player.combat += 2,
                    _ => ()
                }
            },
            Type::Base => {
                match self.card.base_type {
                    base::Type::TheHive => self.player.draw(),
                    _ => ()
                }
            },
            _ => { println!("{} does not have an ally ability.",
                     self.card.name) }
        }
    }
}
