use super::board;
use super::board_iterators;
use super::game;
use std::cmp;

pub fn get_ai_move(game: &game::Game) -> usize {
    let max_player = game.current_player;
    let (ai_move, _) = ab_max(&game, max_player, 6, -10000, 10000);
    ai_move
}

fn end_score(game: &game::Game, max_player: board::Player, depth: usize) -> Option<(usize, i32)> {
    if depth == 0 || game.state != game::GameState::Playing {
        let last_move = game
            .get_board()
            .get_last_move()
            .expect("should not have a winner if no move were made");
        return Some((last_move, calc_score(game, max_player)));
    }

    None
}

fn ab_min(game: &game::Game, max_player: board::Player, depth: usize, alpha: i32, beta: i32) -> (usize, i32) {
    if let Some(score) = end_score(game, max_player, depth) {
        return score;
    }

    let mut beta = beta;
    let mut best_score = 10000;
    let mut best_move = 0;
    for m in game.get_board().get_valid_moves() {
        let mut copy_state = game.clone();
        copy_state.play(m).expect("should be valid move");
        let (_, score) = ab_max(&copy_state, max_player, depth - 1, alpha, beta);
        if score < best_score {
            best_score = score;
            best_move = m;
        }
        beta = cmp::min(beta, best_score);
        if beta <= alpha {
            break;
        }
    }
    (best_move, best_score)
}

fn ab_max(game: &game::Game, max_player: board::Player, depth: usize, alpha: i32, beta: i32) -> (usize, i32) {
    if let Some(val) = end_score(game, max_player, depth) {
        return val;
    }

    let mut alpha = alpha;
    let mut best_score = -10000;
    let mut best_move = 0;
    for m in game.get_board().get_valid_moves() {
        let mut copy_state = game.clone();
        copy_state.play(m).expect("should be valid move");
        let (_, score) = ab_min(&copy_state, max_player, depth - 1, alpha, beta);
        if score > best_score {
            best_score = score;
            best_move = m;
        }
        alpha = cmp::max(alpha, best_score);
        if alpha >= beta {
            break;
        }
    }
    (best_move, best_score)
}

fn calc_score(game: &game::Game, player: board::Player) -> i32 {
    score_columns(game, player)
        + score_rows(game, player)
        + score_diagonals(game, player)
}

fn score_columns(game: &game::Game, player: board::Player) -> i32 {
    let mut max = 0;
    for col in board_iterators::ColIterator::new(game.get_board()) {
        max += calculate_points(col, player);
    }
    max
}

fn score_rows(game: &game::Game, player: board::Player) -> i32 {
    let mut max = 0;
    for col in board_iterators::RowIterator::new(game.get_board()) {
        max += calculate_points(col, player);
    }
    max
}

fn score_diagonals(game: &game::Game, player: board::Player) -> i32 {
    let mut max = 0;
    let diag_it = board_iterators::MainDiagonalIterator::new(game.get_board()).chain(
        board_iterators::SecondaryDiagonalIterator::new(game.get_board()),
    );
    for col in diag_it {
        max += calculate_points(col, player);
    }
    max
}

fn calculate_points(pieces: Vec<&Option<board::Player>>, player: board::Player) -> i32 {
    let mut score = 0;
    for w in pieces.windows(4) {
        score += score_window(w, Some(player));
    }
    score
}

fn score_window(pieces: &[&Option<board::Player>], wanted_piece: Option<board::Player>) -> i32 {
    let mut score = 0;
    let mut op_score = 0;
    let op_player = if let Some(player) = wanted_piece {
        match player {
            board::Player::Yellow => Some(board::Player::Red),
            board::Player::Red => Some(board::Player::Yellow),
        }
    } else {
        None
    };

    for &&p in pieces {
        if p == wanted_piece {
            score += 1;
        } else if p == op_player {
            op_score += 1;
        }
    }

    if op_score == 4 {
        return -10000;
    }

    if score == 4 {
        return 10000;
    }

    score
}
