pub mod zjvoltis;
use crate::zjvoltis::{analyze_moves};

fn main() {
    //let moves = ["i33", "b63"];
	let moves = [];

    // Analyze moves with a depth of 6
    let result = analyze_moves(&moves, 6);

    // Print the analysis result
    println!("Best move: {}", result.best_move.to_string());
    println!("Position before:\n{}", result.position_before);
    println!("Position after:\n{}", result.position_after);
}
