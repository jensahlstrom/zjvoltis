pub mod zjvoltis;
include!("minimax.rs");
use std::sync::mpsc;
use std::time::SystemTime;
fn main() {
    for depth in 1..10 {
        println!("Depth {}", depth);
        let mut game = Zjvoltis::new();
        game = game.make_move(ZjvoltisMove::from_string("c31")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("i62")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("f32")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("i51")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("i32")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("h62")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("f32")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("i41")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("a21")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("e71")).unwrap();
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
}
