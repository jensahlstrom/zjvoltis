use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::zjvoltis::{Zjvoltis, ZjvoltisMove};

fn iterate(game: Zjvoltis, channel: Sender<(i32, i32, Option<ZjvoltisMove>, i32, Duration)>) {
    let mut depth = 1;
    loop {
        let now = SystemTime::now();
        let (score, best_move, nodes) = minimax(&game, depth);
        let _ = channel.send((depth, score, best_move, nodes, now.elapsed().unwrap()));
        depth += 1;
    }
}

fn minimax(game: &Zjvoltis, depth: i32) -> (i32, Option<ZjvoltisMove>, i32) {
    let maximize = game.white_to_move;
    return minimax_ab(game, depth, maximize, i32::MIN, i32::MAX);
}
fn minimax_ab(
    game: &Zjvoltis,
    depth: i32,
    maximize: bool,
    mut alpha: i32,
    mut beta: i32,
) -> (i32, Option<ZjvoltisMove>, i32) {
    if game.game_over.is_some() {
        return (game.game_over.unwrap() * 1000, None, 1);
    }
    if depth == 0 {
        return (game.evaluate(), None, 1);
    }
    let mut best_move = None;
    let mut nodes = 0;
    let mut moves = game.generate_moves();
    moves.sort_by_key(|(_, child)| child.material);
    if maximize {
        moves.reverse();
        let mut best = i32::MIN;
        for (m, child) in moves {
            let (val, _, n) = minimax_ab(&child, depth - 1, false, alpha, beta);
            nodes += n;
            if val > best {
                best_move = Some(m);
                best = val;
            }
            if val > beta {
                break;
            }
            alpha = alpha.max(best);
        }
        return (best, best_move, nodes);
    } else {
        let mut best = i32::MAX;
        for (m, child) in moves {
            let (val, _, n) = minimax_ab(&child, depth - 1, true, alpha, beta);
            nodes += n;
            if val < best {
                best_move = Some(m);
                best = val;
            }
            if val < alpha {
                break;
            }
            beta = beta.min(best);
        }
        return (best, best_move, nodes);
    }
}
