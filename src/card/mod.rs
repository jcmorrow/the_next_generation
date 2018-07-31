// NOTE: enum vals are commented, as they throw warnings when not used
pub mod base;
pub mod ship;
pub mod outpost;

#[derive(Debug)]
pub enum Faction {
    Blob,
    MachineCult,
    TradeFederation,
    StarEmpire,
    Unaligned,
}

#[derive(Debug)]
pub enum CardType {
    Ship,
    Base,
    Outpost,
}

#[derive(Debug)]
pub struct Card {
    pub card_type: CardType,
    pub faction: Faction,
    pub name: String,
    pub cost: i32,
    pub combat: i32,
    pub trade: i32
}
