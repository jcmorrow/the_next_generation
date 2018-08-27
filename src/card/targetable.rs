use std::fmt;

pub trait Targetable: fmt::Display {
    fn is_targetable(&self) -> bool;

    fn receive_combat(&mut self, combat: i32);
}
