use player::Player;

pub trait Targetable {
    fn is_targetable(&self) -> bool;

    fn process_attack(&self, player: Player, combat: i32) -> Player;
}
