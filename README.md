# Chess Library

This is a chess library that does the management of chess games.
It calculates all legal moves for any given position, and handles all events that might
happen during a chess game

# Start a game
This starts a new game session and returns a game struct

```rust
const game = ChessLibrary::Start();
```

# Game struct
This struct holds all important information about the current game

```rust
struct Game {
    board: Board,
    status: Status,
    current_move: Move,
    white_castle_short: bool,
    white_castle_long: bool,
    black_castle_short: bool,
    black_castle_long: bool,
}
```

# Status
This struct contain all important information about the current game status
```rust
pub enum Status {
    WHITE_TO_MOVE,
    BLACK_TO_MOVE,
    DRAW,
    WHITE_HAS_CHECKMATE,
    BLACK_HAS_CHECKMATE,
}
```

# Get Moves
The first function gets all the legal moves for all pieces and maps all legal moves for each piece
to its corresponding position on the board

The second function gets all legal moves for any given position (rank, file)
```rust
use std::collections::HashMap;

const all_legal_moves: HashMap<Position, Vec<Position>> = ChessLibrary::get_all_legal_moves(game);

const legal_moves = Vec<Position> = ChessLibrary::get_legal_moves_for_piece(position);
```

# Make move
This function makes a move and modifies the game struct and returns a boolean
on whether the operation was successful or not

```rust
const success: bool = ChessLibrary::make_move(startPosition, endPosition, game);
```