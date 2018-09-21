// use player::Player;

// pub struct AttackEvent<'a> {
//     combat: i32,
//     player: &'a Player,
//     target: &'a Player,
// }

// impl<'a> AttackEvent<'a> {
//     pub fn new(combat: i32,
//                player: &'a Player,
//                target: &'a Player) -> AttackEvent<'a> {
//         AttackEvent {
//             combat,
//             player,
//             target,
//         }
//     }

//     pub fn log(&mut self) {
//         print!("{} attacks {} for {}\n",
//                self.player.name,
//                self.target.name,
//                self.combat);
//     }
// }
