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
    print!("{:#}", player_1);

    print!("{:#}", player_2);
    player_2.play();
    player_2.play();
    player_2.play();
    player_2.play();
    player_2.play();
    print!("{:#}", player_2);
}
