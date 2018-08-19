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

    let scout = Scout::new();
    let viper = Viper::new();
    let battle_station = BattleStation::new();

    print!("{:#}", player_1);
    player_1 = scout.play(player_1);
    player_1 = viper.play(player_1);
    print!("{:#}", player_1);
    player_1 = scout.play(player_1);
    player_1 = viper.play(player_1);
    print!("{:#}", player_1);
    player_2 = battle_station.play(player_2);
    player_2 = player_1.process_attack(player_2, player_1.combat);
    print!("{:#}", player_2);
}
