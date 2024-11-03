# konane
A lame attempt at learning rust by implementing a game engine for [Konane](https://en.wikipedia.org/wiki/K%C5%8Dnane).

## How it works

Any position in the game is evaluated using a primitive evaluation function:
the difference between the number of legal moves which the player and his
opponent has in a given position. Then, an alpha-beta pruning algorithm with
a fixed depth is applied, which finds the best move.

## How to use it.

Why would you use it? It's single threaded, primitive and crappy.

### Compilation

Compile the program using Cargo:
```bash
cargo build --release
```

### Input file

The program requires a text file containing a valid Konane position. Examples can be found in the `examples/` directory.

Example of `examples/input1.txt`:
```
xox.xo
oxo.ox
xox.xo
oxo.ox
xoxoxo
oxo.ox
```

In this representation:

* Each line corresponds to a row on the board.
* `x` represents a piece belonging to the first player.
* `o` represents a piece belonging to the second player.
* `.` represents an empty square.

### Running the program

Execute the program with the following syntax:
```
./target/release/konane <depth> <input file> [single move?]
```

* &lt;depth&gt;: The number of plies the alpha-beta pruning algorithm will search.
* <input file>: The path to the input file containing the game board.
* [single move?]: (Optional) If set to 1, the program will find the next best move and stop. If omitted, the program will play the entire game against itsel


### Example

```
$ ./target/release/konane 8 examples/input1.txt 1
Loading position from examples/input1.txt
Eval: 1, leaves visited: 869232, nodes: 1189238, cache hits: 397714, cache_length: 625245
-------------
|X|O|X|.|X|O|
|O|X|O|.|O|X|
|X|O|X|.|X|O|
|O|X|O|.|O|X|
|X|O|X|O|X|O|
|O|.|.|X|O|X|
-------------
b1 -> d1
```

* Eval: The evaluation score of the position. Positive values favor the first player; negative values favor the second player.
* Board Coordinates:
```
6......
5......
4......
3......
2......
1......
 abcdef
```

which means that the move suggested by the engine (b1 -> d1) looks like the following:
```
......      ......
......      ......
......  ->  ......
......      ......
......      ......
.X....      ...X..
```
* Statistics: The numbers following the evaluation provide insights into the performance of the alpha-beta pruning algorithm, useful for further optimization.

## Future Improvements

1. Implement argument parsing.
1. Refactor code for better readability and maintainability.
1. Add basic unit tests.
1. Implement concurrency to improve performance (currently single-threaded).

But most likely, it will just be abandoned as is.

