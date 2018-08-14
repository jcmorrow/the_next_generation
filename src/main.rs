use player::Player;
use card::ship::Scout;
use card::Card;

mod card;
mod player;

fn main() {
    let mut player_1 = Player::new("Cameron");

    let scout = Scout::new();

    print!("{:#}", player_1);
    player_1 = scout.play(&player_1);
    print!("{:#}", player_1);
    player_1 = scout.play(&player_1);
    print!("{:#}", player_1);
}
