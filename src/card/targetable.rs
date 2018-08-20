use player::Player;

pub trait Targetable {
    fn is_targetable(&self) -> bool;

    fn receive_combat(&self, player: Player, combat: i32) -> Player;
}
