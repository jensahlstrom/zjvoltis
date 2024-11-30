pub mod zjvoltis;
include!("minimax.rs");
use std::sync::mpsc;
use std::time::SystemTime;
fn main() {
    //let moves = ["i33", "h62", "h42"];
    let moves = [];
    let mut game = Zjvoltis::new();
    for m in moves {
        game = game.make_move(ZjvoltisMove::from_string(m)).unwrap();
    }
    let old_game = game;
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        iterate(game, tx);
    });
    loop {
        let (depth, score, m, nodes, elapsed) = rx.recv().unwrap();
        println!("{}", old_game.to_string());
        let new_game = old_game.make_move(m.unwrap()).unwrap();
        println!("{}", new_game.to_string());
        println!(
            "Depth: {}  Score: {}  Best move: {}",
            depth,
            score,
            m.unwrap().to_string()
        );
        println!(
            "Nodes: {}  Time: {}  N/s: {}",
            nodes,
            elapsed.as_millis(),
            (nodes as f64 / elapsed.as_secs_f64()).round()
        );
    }
}
