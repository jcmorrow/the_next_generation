use std::fmt;

use effect::Effect;
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
    StarEmpire,
    TradeFederation,
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
    Battlecruiser,
    BattleBlob,
    BattleMech,
    BattlePod,
    BlobCarrier,
    BlobDestroyer,
    BlobFighter,
    Corvette,
    Cutter,
    Dreadnaught,
    EmbassyYacht,
    Explorer,
    Flagship,
    Freighter,
    ImperialFighter,
    ImperialFrigate,
    MissleBot,
    MissleMech,
    Mothership,
    NoShipType,
    PatrolMech,
    Ram,
    Scout,
    SupplyBot,
    SurveyShip,
    TradeBot,
    TradeEscort,
    TradePod,
    Viper,
}

impl Default for ShipType {
    fn default() ->  ShipType { ShipType::NoShipType }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum OutpostType {
    BattleStation,
    BrainWorld,
    DefenseCenter,
    Junkyard,
    MachineBase,
    MechWorld,
    PortOfCall,
    RecyclingStation,
    RoyalRedoubt,
    SpaceStation,
    TradingPost,
    WarWorld,
    NoOutpostType,
}

impl Default for OutpostType {
    fn default() ->  OutpostType { OutpostType::NoOutpostType }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BaseType {
    BarterWorld,
    BlobWheel,
    BlobWorld,
    CentralOffice,
    FleetHQ,
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
            Faction::StarEmpire => "Star Empire",
            Faction::TradeFederation => "Trade Federation",
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
    pub ally_effects: Vec<Effect>,
    pub base_type: BaseType,
    pub card_type: CardType,
    pub cost: i32,
    pub effects: Vec<Effect>,
    pub faction: Faction,
    pub has_used_ally_ability: bool,
    pub health: i32,
    pub name: String,
    pub outpost_type: OutpostType,
    pub scrap_abilities: Vec<Choice>,
    pub scrap_effects: Vec<Effect>,
    pub ship_type: ShipType,
}

impl Card {
    pub fn run(&mut self,
               player: &mut Player,
               opponents: &[&mut Player],
               trade_row: &mut TradeRow) {
        player.choices.extend(self.abilities.clone());
        player.effects.extend(self.effects.clone());

        let fleet_hq_played = player.in_play.iter().any(|c| c.base_type == BaseType::FleetHQ);
        // Conditional ability triggers
        if fleet_hq_played {
            match self.card_type {
                CardType::Ship => {
                    player.effects.push(Effect::GainCombat(1));
                },
                _ => ()
            }
        }
        match self.ship_type {
            ShipType::EmbassyYacht => {
                if player.in_play.iter()
                                 .filter(
                                     |c| c.card_type == CardType::Base ||
                                     c.card_type == CardType::Outpost
                                 ).count() >= 2 {
                    player.effects.extend(vec!(Effect::Draw, Effect::Draw));
                }
            },
            _ => ()
        }

        for card in &mut player.in_play {
            if (card.faction == self.faction || card.outpost_type == OutpostType::MechWorld)
                && !card.has_used_ally_ability
            {
                card.has_used_ally_ability = true;
                player.choices.extend(card.ally_abilities.clone());
                player.effects.extend(card.ally_effects.clone());
            }
        }
        if player.has_ally_in_play(&self.faction) {
            self.has_used_ally_ability = true;
            player.choices.extend(self.ally_abilities.clone());
            player.effects.extend(self.ally_effects.clone());
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}
