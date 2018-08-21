extern crate rand;

use player::Player;

use card::ship::Scout;
use card::ship::Viper;
use card::outpost::BattleStation;
use card::targetable::Targetable;
use card::Card;

mod card;
mod player;

fn main() {
    let mut player_1 = Player::new("Cameron");
    let mut player_2 = Player::new("Josh");

    print!("{:#}", player_1);
    player_1.play();
    player_1.play();
    player_1.play();
    player_1.play();
    player_1.play();

    while player_2.authority > 0 {
        let mut opponents = Vec::new();
        opponents.push(&mut player_2);
        player_1.attack(opponents.as_mut_slice());
    }
    print!("{:#}", player_1);

    print!("{:#}", player_2);
    player_2.play();
    player_2.play();
    player_2.play();
    player_2.play();
    player_2.play();
    print!("{:#}", player_2);
}
