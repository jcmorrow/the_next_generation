use card::Card;
use card::base;
use card::outpost;
use card::ship;

use std::fmt;
use rand::{thread_rng, Rng};

pub struct TradeRow {
    pub deck: Vec<Card>,
    pub face_up: Vec<Card>,
    pub scrapped: Vec<Card>
}

impl TradeRow {
    pub fn get_card(&mut self, index: usize) -> Card {
        let card = self.face_up.remove(index);
        match index {
            // TODO: panic central if the trade row deck runs out
            5 => self.face_up.insert(5, ship::explorer()),
            i => {
                match self.deck.pop() {
                    Some(card) => self.face_up.insert(i, card),
                    None => (),
                }
            }
        }
        card
    }

    pub fn new() -> TradeRow {
        let mut trade_row = TradeRow{
            deck: Vec::new(),
            face_up: Vec::new(),
            scrapped: Vec::new()
        };

        // Blob
        for _n in 0..1 { trade_row.deck.push(ship::battle_blob()); }
        for _n in 0..2 { trade_row.deck.push(ship::battle_pod()); }
        for _n in 0..1 { trade_row.deck.push(ship::blob_carrier()); }
        for _n in 0..2 { trade_row.deck.push(ship::blob_destroyer()); }
        for _n in 0..3 { trade_row.deck.push(ship::blob_fighter()); }
        for _n in 0..3 { trade_row.deck.push(base::blob_wheel()); }
        for _n in 0..1 { trade_row.deck.push(base::blob_world()); }
        for _n in 0..1 { trade_row.deck.push(ship::mothership()); }
        for _n in 0..2 { trade_row.deck.push(ship::ram()); }
        for _n in 0..1 { trade_row.deck.push(base::the_hive()); }
        for _n in 0..3 { trade_row.deck.push(ship::trade_pod()); }

        // Machine Cult
        for _n in 0..1 { trade_row.deck.push(ship::battle_mech()); }
        for _n in 0..2 { trade_row.deck.push(outpost::battle_station()); }
        for _n in 0..1 { trade_row.deck.push(outpost::brain_world()); }
        for _n in 0..1 { trade_row.deck.push(outpost::junkyard()); }
        for _n in 0..1 { trade_row.deck.push(outpost::machine_base()); }
        for _n in 0..1 { trade_row.deck.push(outpost::mech_world()); }
        for _n in 0..3 { trade_row.deck.push(ship::missle_bot()); }
        for _n in 0..1 { trade_row.deck.push(ship::missle_mech()); }
        for _n in 0..2 { trade_row.deck.push(ship::patrol_mech()); }
        for _n in 0..3 { trade_row.deck.push(ship::supply_bot()); }
        for _n in 0..3 { trade_row.deck.push(ship::trade_bot()); }
        for _n in 0..1 { trade_row.deck.push(ship::stealth_needle()); }

        // Star Empire
        for _n in 0..1 { trade_row.deck.push(ship::battlecruiser()); }
        for _n in 0..2 { trade_row.deck.push(ship::corvette()); }
        for _n in 0..1 { trade_row.deck.push(ship::dreadnaught()); }
        for _n in 0..1 { trade_row.deck.push(base::fleet_hq()); }
        for _n in 0..3 { trade_row.deck.push(ship::imperial_fighter()); }
        for _n in 0..3 { trade_row.deck.push(ship::imperial_frigate()); }
        for _n in 0..2 { trade_row.deck.push(outpost::recycling_station()); }
        for _n in 0..1 { trade_row.deck.push(outpost::royal_redoubt()); }
        for _n in 0..2 { trade_row.deck.push(outpost::space_station()); }
        for _n in 0..3 { trade_row.deck.push(ship::survey_ship()); }
        for _n in 0..1 { trade_row.deck.push(outpost::war_world()); }

        // Trade Federation
        for _n in 0..2 { trade_row.deck.push(base::barter_world()); }
        for _n in 0..1 { trade_row.deck.push(base::central_office()); }
        for _n in 0..1 { trade_row.deck.push(ship::command_ship()); }
        for _n in 0..3 { trade_row.deck.push(ship::cutter()); }
        for _n in 0..1 { trade_row.deck.push(outpost::defense_center()); }
        for _n in 0..2 { trade_row.deck.push(ship::embassy_yacht()); }
        for _n in 0..3 { trade_row.deck.push(ship::federation_shuttle()); }
        for _n in 0..1 { trade_row.deck.push(ship::flagship()); }
        for _n in 0..2 { trade_row.deck.push(ship::freighter()); }
        for _n in 0..1 { trade_row.deck.push(outpost::port_of_call()); }
        for _n in 0..1 { trade_row.deck.push(ship::trade_escort()); }
        for _n in 0..2 { trade_row.deck.push(outpost::trading_post()); }

        thread_rng().shuffle(&mut trade_row.deck);

        for _n in 0..5 {
            trade_row.face_up.push(trade_row.deck.pop().unwrap());
        }
        trade_row.face_up.insert(5, ship::explorer());
        trade_row
    }
}

impl fmt::Display for TradeRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trade Row:\n").unwrap();
        for card in self.face_up.iter() {
            write!(f, "  {}", card).unwrap();
        }
        write!(f, "\n")
    }
}
