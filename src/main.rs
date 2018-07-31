use card::Card;
use player::Player;

mod card;
mod player;

fn main() {
    let player_1 = Player::new("Cameron");
    let player_2 = Player::new("Josh");

    let scout = Card::scout();
    let viper = Card::viper();
    let trade_bot = Card::trade_bot();
    let blob_fighter = Card::blob_fighter();
    let barter_world = Card::barter_world();
    let war_world = Card::war_world();

    print!("{:#}", player_1);
    print!("{:#}", player_2);

    print!("{:#}", scout);
    print!("{:#}", viper);
    print!("{:#}", trade_bot);
    print!("{:#}", blob_fighter);
    print!("{:#}", barter_world);
    print!("{:#}", war_world);
}
