Zjvoltis minimax search implementation

Files:
main.rs - main file for the program, calls the iterative deepening function in a separate thread and then prints out the results as they come
minimax.rs - minimax search algorithm, iterate functions runs iterative deepening (searching first at 0 depth, then at 1 and so on)
zjvoltis.rs - contains the game data structure and logic

Game is represented as a 10x10 array with a u8 value for each square. For efficiency of evaluation it also keeps track of material count.

There is also conversion from a "FEN" variant of representing the board. It is similar to the chess FEN format. The initial position looks like this in Zjvoltis FEN:
"i1ll1jj1oo/i1zl1js1oo/izzltjssvv/iz1ttt1sv1/A/A/1VS1TTT1ZI/VVSSJTLZZI/OO1SJ1LZ1I/OO1JJ1LL1I w"

Enjoy!