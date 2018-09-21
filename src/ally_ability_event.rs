// use card::BaseType;
// use card::Card;
// use card::CardType;
// use card::ShipType;
// use player::Player;

// pub struct AllyAbilityEvent<'a> {
//     player: &'a mut Player,
//     card: &'a Card,
// }

// impl<'a> AllyAbilityEvent<'a> {
//     pub fn new(card: &'a Card,
//                player: &'a mut Player) -> AllyAbilityEvent<'a> {
//         AllyAbilityEvent { card: card, player: player }
//     }

//     pub fn trigger_ability(&mut self) {

//         print!("{} uses the ally ability of {}\n",
//                self.player.name,
//                self.card.name);

//         match self.card.card_type {
//             CardType::Ship => {
//                 match self.card.ship_type {
//                     ShipType::BattlePod => self.player.combat += 2,
//                     _ => ()
//                 }
//             },
//             CardType::Base => {
//                 match self.card.base_type {
//                     BaseType::TheHive => self.player.draw(),
//                     _ => ()
//                 }
//             },
//             _ => { println!("{} does not have an ally ability.",
//                      self.card.name) }
//         }
//     }
// }
