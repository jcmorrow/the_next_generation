use card::Card;
use card::BaseType;
use player::Player;

pub struct AllyAbilityEvent<'a> {
    player: &'a mut Player,
    card: &'a Card,
}

impl<'a> AllyAbilityEvent<'a> {
    pub fn new(card: &'a Card, player: &'a mut Player) -> AllyAbilityEvent<'a> {
        AllyAbilityEvent {
            card: card,
            player: player,
        }
    }

    pub fn trigger_ability(&mut self) {
        // print!("{} plays {}\n", self.player.name, self.card.name);

        match self.card.base_type {
            BaseType::TheHive => { self.player.draw(); }
            _ => { println!("{} does not have an ally ability.", self.card.name)}
        }
    }
}
