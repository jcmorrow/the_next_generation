
mod card;

mod player;

use card::Card;

use player::Player;

fn main() {
    let player_1 = Player::new("Cameron");
    let player_2 = Player::new("Josh");

    let scout_1 = Card::scout();
    let viper_1 = Card::viper();
    
    print!("{:?}", player_1);
    print!("{:?}", player_2);
    print!("{:?}", scout_1);
    print!("{:?}", viper_1);
}
