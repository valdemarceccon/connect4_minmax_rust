use super::board;

pub trait BoardEvaluator {
    fn find_player_sequence(&self, player: board::Player, size: usize);
}
