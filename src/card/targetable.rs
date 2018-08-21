use player::Player;

pub trait Targetable {
    fn is_targetable(&self) -> bool;

    fn receive_combat(&mut self, combat: i32);
}
