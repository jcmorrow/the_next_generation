use std::fmt;

use player::Player;
use trade_row::TradeRow;
use choice::Choice;

pub mod base;
pub mod ship;
pub mod outpost;
pub mod targetable;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Faction {
    Blob,
    MachineCult,
    Unaligned,
}

impl Default for Faction {
    fn default() -> Faction { Faction::Unaligned }
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum CardType {
    NoCardType,
    Outpost,
    Ship,
    Base
}

impl Default for CardType {
    fn default() ->  CardType { CardType::NoCardType }
}

#[derive(Clone)]
#[derive(Debug)]
pub enum ShipType {
    BattleBlob,
    BattlePod,
    BlobCarrier,
    Explorer,
    NoShipType,
    Scout,
    Viper,
}

impl Default for ShipType {
    fn default() ->  ShipType { ShipType::NoShipType }
}

#[derive(Clone)]
#[derive(Debug)]
pub enum OutpostType {
    BattleStation,
    NoOutpostType,
}

impl Default for OutpostType {
    fn default() ->  OutpostType { OutpostType::NoOutpostType }
}

#[derive(Clone)]
#[derive(Debug)]
pub enum BaseType {
    TheHive,
    NoBaseType,
}

impl Default for BaseType {
    fn default() ->  BaseType { BaseType::NoBaseType }
}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Faction::Blob => "Blob",
            Faction::MachineCult => "Machine Cult",
            Faction::Unaligned => "Unaligned",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            CardType::Ship => "Ship",
            CardType::Outpost => "Outpost",
            CardType::Base => "Base",
            CardType::NoCardType => "NoCardType",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct Card {
    pub abilities: Vec<Choice>,
    pub ally_abilities: Vec<Choice>,
    pub base_type: BaseType,
    pub card_type: CardType,
    pub combat: i32,
    pub cost: i32,
    pub faction: Faction,
    pub health: i32,
    pub name: String,
    pub ship_type: ShipType,
    pub outpost_type: OutpostType,
    pub trade: i32,
    pub scrappable: bool,
    pub has_ally_ability: bool,
    pub has_used_ally_ability: bool
}

impl Card {
    pub fn run(&mut self,
               player: &mut Player,
               opponents: &[&mut Player],
               trade_row: &mut TradeRow) {
        match self.trade {
            0 => (),
            n => player.trade += n,
        }
        match self.combat {
            0 => (),
            n => player.combat += n,
        }
        player.choices.extend(self.abilities.clone());

        for card in &mut player.bases {
            if card.faction == self.faction && !card.has_used_ally_ability
            {
                card.has_used_ally_ability = true;
                player.choices.extend(card.ally_abilities.clone())
            }
        }
        if player.has_allies_in_play(&self.faction, 1) {
            self.has_used_ally_ability = true;
            player.choices.extend(self.ally_abilities.clone());
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}
