use player::Player;
use std::slice;

pub struct Players {
    players: Vec<Player>
}

impl Players {
    fn current_and_opponents(&mut self, current: isize) -> (&mut Player,
                                                            &mut [Player],
                                                            &mut [Player]) {
        let len = self.players.len();
        let ptr = self.players.as_mut_ptr();
        assert!(current as usize <= len);
        unsafe {
            (
                &mut slice::from_raw_parts_mut(
                    ptr.offset(current),
                    1,
                )[0],
                slice::from_raw_parts_mut(ptr, current as usize),
                slice::from_raw_parts_mut(
                    ptr.offset(current + 1),
                    len - (current as usize) - 1,
                )
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use player::Player;
    use players::Players;

    #[test]
    fn players_rotation() {
        let player_1: Player = Player::new("Player #1");
        let player_2: Player = Player::new("Player #2");
        let player_3: Player = Player::new("Player #3");

        let mut players = Players {players: vec!(player_1, player_2, player_3)};

        let (current, before, after) = players.current_and_opponents(0);

        assert_eq!(current.name, "Player #1");
        assert_eq!(before.len(), 0);
        assert_eq!(after.len(), 2);
    }
}
