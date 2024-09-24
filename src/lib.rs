use std::cmp::PartialEq;
use std::collections::HashMap;
use serde::Serialize;
use crate::Color::{BLACK, WHITE};
use crate::Move::{CAPTURE, REGULAR};
use crate::Status::{BLACK_TO_MOVE, WHITE_TO_MOVE};
use std::io::*;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

#[derive(PartialEq, Debug)]
pub enum Status {
    WHITE_TO_MOVE,
    BLACK_TO_MOVE,
    DRAW,
    WHITE_HAS_CHECKMATE,
    BLACK_HAS_CHECKMATE,
}

pub type ChessBoard = Vec<Vec<char>>;

#[derive(Debug)]
pub struct Board {
    pub board: ChessBoard,
}

impl Board {
    pub fn create() -> Board {
        Board {
            board: convert_fen_to_vector(STARTING_FEN),
        }
    }

    pub fn create_from_fen(FEN: &str) -> Board {
        Board {
            board: convert_fen_to_vector(FEN),
        }
    }

    pub fn get(&self, rank: usize, file: usize) -> char {
        self.board[rank][file]
    }

    pub fn pushRow(&mut self, row: Vec<char>) {
        self.board.push(row);
    }

    pub fn clone(&self) -> Self {
        // Create a new Board instance
        Board {
            board: self.board.clone(), // Clone the 2D vector
        }
    }

    pub fn make_move(board: &Board, start: Position, end: &Position, piece: char) -> Board {
        let mut new_board = board.clone();
        new_board.board[start.rank][start.file] = '-';
        new_board.board[end.rank][end.file] = piece;
        new_board
    }
    /*
    pub fn generate_legal_moves(&self, currentTurn: &Color) -> Vec<Board> {
        let mut boards: Vec<Board> = Vec::new();

        for (rowIndex, row) in self.board.iter().enumerate() {
            for (fileIndex, piece) in row.iter().enumerate() {
                if *currentTurn == WHITE {
                    match piece {
                        &'P' => {
                            let legal_moves = get_pawn_moves(self, &Position::create(rowIndex, fileIndex));

                            for legal_move in &legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), legal_move, 'P');
                                boards.push(new_board);
                            }
                        }
                        &'R' => {
                            let legal_moves = get_rook_moves(self, &Position::create(rowIndex, fileIndex));
                            for legal_move in legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), &legal_move, 'R');
                                boards.push(new_board);
                            }
                        }
                        &'N' => {
                            let legal_moves = get_knight_moves(self, &Position::create(rowIndex, fileIndex));
                            for legal_move in legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), &legal_move, 'N');
                                boards.push(new_board);
                            }
                        }
                        &'B' => {
                            let legal_moves = get_bishop_moves(self, &Position::create(rowIndex, fileIndex));
                            for legal_move in legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), &legal_move, 'B');
                                boards.push(new_board);
                            }
                        }
                        &'Q' => {
                            let legal_moves = get_queen_moves(self, &Position::create(rowIndex, fileIndex));
                            for legal_move in legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), &legal_move, 'Q');
                                boards.push(new_board);
                            }
                        }
                        &'K' => {
                            let legal_moves = get_king_moves(self, &Position::create(rowIndex, fileIndex), WHITE, Game::new());
                            for legal_move in legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), &legal_move, 'K');
                                boards.push(new_board);
                            }
                        }
                        _ => {}
                    }
                }

                else {
                    match piece {
                        &'p' => {
                            let legal_moves = get_pawn_moves(self, &Position::create(rowIndex, fileIndex));

                            for legal_move in &legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), legal_move, 'p');
                                boards.push(new_board);
                            }
                        }
                        &'r' => {
                            let legal_moves = get_rook_moves(self, &Position::create(rowIndex, fileIndex));
                            for legal_move in legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), &legal_move, 'r');
                                boards.push(new_board);
                            }
                        }
                        &'n' => {
                            let legal_moves = get_knight_moves(self, &Position::create(rowIndex, fileIndex));
                            for legal_move in legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), &legal_move, 'n');
                                boards.push(new_board);
                            }
                        }
                        &'b' => {
                            let legal_moves = get_bishop_moves(self, &Position::create(rowIndex, fileIndex));
                            for legal_move in legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), &legal_move, 'b');
                                boards.push(new_board);
                            }
                        }
                        &'q' => {
                            let legal_moves = get_queen_moves(self, &Position::create(rowIndex, fileIndex));
                            for legal_move in legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), &legal_move, 'q');
                                boards.push(new_board);
                            }
                        }
                        &'k' => {
                            let legal_moves = get_king_moves(self, &Position::create(rowIndex, fileIndex));
                            for legal_move in legal_moves {
                                let new_board = Self::make_move(self, Position::create(rowIndex, fileIndex), &legal_move, 'k');
                                boards.push(new_board);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        boards
    }
     */

    pub fn print(&self) {
        for row in &self.board {
            println!("{:?}", row);
        }
    }

    /*
    pub fn perft(&self, depth: usize, currentColor: Color) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;

        let moves = self.generate_legal_moves(&currentColor);

        // Recursively count nodes at the next depth for each legal move
        for new_board in moves {
            let newColor = if currentColor == WHITE {
                BLACK
            }
            else {
                WHITE
            };

            nodes += new_board.perft(depth - 1, newColor);

            /*
            new_board.print();
            println!("");
            println!("");
             */
        }

        nodes
    }
     */
}

#[derive(Serialize)]
#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
pub struct Position {
    rank: usize,
    file: usize
}

impl Position {
    pub fn create(rank: usize, file: usize) -> Position {
        Position {
            rank,
            file
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Color {
    WHITE,
    BLACK
}

pub enum Move {
    REGULAR,
    CAPTURE,
    CASTLE
}
pub struct Game {
    board: Board,
    status: Status,
    current_move: Move,
    white_castle_short: bool,
    white_castle_long: bool,
    black_castle_short: bool,
    black_castle_long: bool,
    en_passant_possible: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            status: WHITE_TO_MOVE,
            current_move: REGULAR,
            white_castle_short: true,
            white_castle_long: true,
            black_castle_short: true,
            black_castle_long: true,
            en_passant_possible: false,
            board: Board::create(),
        }
    }
}

pub fn convert_fen_to_vector(fen: &str) -> ChessBoard {
    let mut board: ChessBoard = Vec::new();
    let position = fen.split(" ").collect::<Vec<&str>>()[0];

    for rank in position.split("/").collect::<Vec<&str>>() {
        let mut row_vector = Vec::new();

        for square in rank.chars() {
            match square.to_digit(10) {
                Some(num) => {
                    let num = num as i32;

                    for _i in 0..num {
                        row_vector.push('-');
                    }
                }

                None => row_vector.push(square)
            }
        }

        board.push(row_vector);
    }

    board
}

fn get_piece_from_position(board: &Board, piece_pos: &Position) -> char {
    board.get(piece_pos.rank, piece_pos.file)
}

fn get_castling_position(game: &Game) -> Vec<Position> {
    let mut positons = Vec::new();

    if game.status == WHITE_TO_MOVE {
        if game.white_castle_short {
            if game.board.get(7, 5) == '-' && game.board.get(7, 6) == '-' {
                positons.push(Position {
                    rank: 7,
                    file: 6
                })
            }
        }

        if game.white_castle_short {
            if game.board.get(7, 1) == '-' && game.board.get(7, 2) == '-' && game.board.get(7, 3) == '-' {
                positons.push(Position {
                    rank: 7,
                    file: 2
                })
            }
        }
    }

    else {
        if game.black_castle_short {
            if game.board.get(0, 5) == '-' && game.board.get(0, 6) == '-' {
                positons.push(Position {
                    rank: 0,
                    file: 6
                })
            }
        }

        if game.black_castle_long {
            if game.board.get(0, 1) == '-' && game.board.get(0, 2) == '-' && game.board.get(0, 3) == '-' {
                positons.push(Position {
                    rank: 0,
                    file: 2
                })
            }
        }
    }

    positons
}

fn has_enemy_piece(board: &Board, pos: &Position, current_piece: char) -> bool {
    let piece = get_piece_from_position(&board, &pos);

    if current_piece.is_uppercase() && piece.is_lowercase() {
        return true;
    }

    if current_piece.is_lowercase() && piece.is_uppercase() {
        return true;
    }

    false
}

fn get_pawn_capture_pos(board: &Board, pawn_pos: &Position, pawn: char) -> Vec<Position> {
    let mut target: Vec<Position> = Vec::new();
    let mut target_rank = 0;

    if pawn.is_lowercase() {
        target_rank = pawn_pos.rank + 1;
    }

    else {
        target_rank = pawn_pos.rank - 1;
    }

    if (pawn_pos.file > 0) {
        let square2 = pawn_pos.file - 1;
        let target2 = Position::create(target_rank, square2);
        let has_enemy2 = has_enemy_piece(&board, &target2, pawn);

        if pawn_pos.file == 7 && has_enemy2 {
            target.push(Position::create(target_rank, square2));
        }

        else if has_enemy2 {
            target.push(Position::create(target_rank, square2))
        }
    }

    if (pawn_pos.file < 7) {
        let square1 = pawn_pos.file + 1;
        let target1 = Position::create(target_rank, square1);
        let has_enemy1 = has_enemy_piece(&board, &target1, pawn);

        if pawn_pos.file == 0 && has_enemy1 {
            target.push(Position::create(target_rank, square1));
        }

        else if has_enemy1 {
            target.push(Position::create(target_rank, square1));
        }
    }

    target
}

fn get_horizontal_moves(board: &Board, piece_pos: &Position) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    let rank = piece_pos.rank;
    let mut file = piece_pos.file;

    loop {
        if file == 7 {
            if rank != piece_pos.rank || file != piece_pos.file {
                positions.push(Position::create(rank, 7));
            }

            file = piece_pos.file;

            break;
        }

        if rank == piece_pos.rank && file == piece_pos.file {
            file += 1;
            continue;
        }

        if board.get(rank, file) != '-'  {
            if is_enemy(board, piece_pos, &Position::create(rank, file)) {
                positions.push(Position::create(rank, file));
            }

            file = piece_pos.file;

            break;
        }

        positions.push(Position::create(rank, file));

        file += 1;
    }

    loop {
        if file == 0 {
            if rank != piece_pos.rank || file != piece_pos.file {
                positions.push(Position::create(rank, 0));
            }

            file = piece_pos.file;

            break;
        }

        if rank == piece_pos.rank && file == piece_pos.file {
            file -= 1;
            continue;
        }

        if board.get(rank, file) != '-' {
            if is_enemy(board, piece_pos, &Position::create(rank, file)) {
                positions.push(Position::create(rank, file));
            }

            file = piece_pos.file;

            break;
        }

        positions.push(Position::create(rank, file));

        file -= 1;
    }

    positions
}

fn is_enemy(board: &Board, current_pos: &Position, target_pos: &Position) -> bool {
    let current_piece = board.get(current_pos.rank, current_pos.file);

    if current_piece.is_lowercase() {
        if board.get(target_pos.rank, target_pos.file).is_uppercase() {
            return true;
        }
    }

    else {
        if board.get(target_pos.rank, target_pos.file).is_lowercase() {
            return true;
        }
    }

    false
}

fn get_vertical_moves(board: &Board, piece_pos: &Position) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    let mut rank = piece_pos.rank;
    let file = piece_pos.file;

    loop {
        if rank == 7 {
            if file != piece_pos.file || rank != piece_pos.rank {
                positions.push(Position::create(7, file));
            }

            rank = piece_pos.rank;

            break;
        }

        if rank == piece_pos.rank && file == piece_pos.file {
            rank += 1;
            continue;
        }

        if board.get(rank, file) != '-' {
            if is_enemy(board, piece_pos, &Position::create(rank, file)) {
                positions.push(Position::create(rank, file));
            }

            rank = piece_pos.rank;

            break;
        }

        positions.push(Position::create(rank, file));

        rank += 1;
    }

    loop {
        if rank == 0 {
            if file != piece_pos.file || rank != piece_pos.rank {
                positions.push(Position::create(rank, file));
            }

            rank = piece_pos.rank;

            break;
        }

        if rank == piece_pos.rank && file == piece_pos.file {
            rank -= 1;
            continue;
        }

        if board.get(rank, file) != '-' {
            if is_enemy(board, piece_pos, &Position::create(rank, file)) {
                positions.push(Position::create(rank, file));
            }

            rank = piece_pos.rank;

            break;
        }

        positions.push(Position::create(rank, file));

        rank -= 1;
    }

    positions
}

fn get_diagonal_moves(board: &Board, piece_pos: &Position) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    let mut rank = piece_pos.rank;
    let mut file = piece_pos.file;

    if rank > 0 && file > 0 {
        loop {
            if board.get(rank - 1, file - 1) != '-' {
                if is_enemy(board, piece_pos, &Position::create(rank - 1, file - 1)) {
                    positions.push(Position::create(rank - 1, file - 1));
                }

                rank = piece_pos.rank;
                file = piece_pos.file;
                break;
            }

            positions.push(Position::create(rank - 1, file - 1));

            rank = rank - 1;
            file = file - 1;

            if rank == 0 || file == 0 {
                rank = piece_pos.rank;
                file = piece_pos.file;
                break;
            }
        }
    }

    if rank > 0 && file < 7 {
        loop {
            if board.get(rank - 1, file + 1) != '-' {
                if is_enemy(board, piece_pos, &Position::create(rank - 1, file + 1)) {
                    positions.push(Position::create(rank - 1, file + 1));
                }

                rank = piece_pos.rank;
                file = piece_pos.file;
                break;
            }

            positions.push(Position::create(rank - 1, file + 1));

            rank = rank - 1;
            file = file + 1;

            if rank == 0 || file == 7 {
                rank = piece_pos.rank;
                file = piece_pos.file;
                break;
            }
        }
    }

    if rank < 7 && file > 0 {
        loop {
            if board.get(rank + 1, file - 1) != '-' {
                if is_enemy(board, piece_pos, &Position::create(rank + 1, file - 1)) {
                    positions.push(Position::create(rank + 1, file - 1));
                }

                rank = piece_pos.rank;
                file = piece_pos.file;
                break;
            }

            positions.push(Position::create(rank + 1, file - 1));

            rank = rank + 1;
            file = file - 1;

            if rank == 7 || file == 0 {
                rank = piece_pos.rank;
                file = piece_pos.file;
                break;
            }
        }
    }

    if rank < 7 && file < 7 {
        loop {
            if board.get(rank + 1, file + 1) != '-' {
                if is_enemy(board, piece_pos, &Position::create(rank + 1, file + 1)) {
                    positions.push(Position::create(rank + 1, file + 1));
                }

                rank = piece_pos.rank;
                file = piece_pos.file;
                break;
            }

            positions.push(Position::create(rank + 1, file + 1));

            rank = rank + 1;
            file = file + 1;

            if rank == 7 || file == 7 {
                rank = piece_pos.rank;
                file = piece_pos.file;
                break;
            }
        }
    }

    positions
}

// Exported Functions

pub fn get_pawn_moves(game: &Game, pawn_pos: &Position) -> Vec<Position> {
    let piece = get_piece_from_position(&game.board, &pawn_pos);

    let mut positions: Vec<Position> = Vec::new();
    println!("{:?}", pawn_pos);
    if piece == 'p' {
        if (game.board.get(pawn_pos.rank + 1,pawn_pos.file) == '-') {
            if pawn_pos.rank == 1 && game.board.get(3,pawn_pos.file) == '-' {
                positions.push(Position::create(3, pawn_pos.file)); // 2 step pawn move
            }

            positions.push(Position::create(pawn_pos.rank + 1, pawn_pos.file)); // 1 step pawn move

            let capture_positions = get_pawn_capture_pos(&game.board, pawn_pos, 'p');

            for capture in capture_positions {
                positions.push(capture);
            }

            return positions;
        }
    } else if piece == 'P' {
        if game.board.get(pawn_pos.rank - 1, pawn_pos.file) == '-' {
            if pawn_pos.rank == 6 && game.board.get(4, pawn_pos.rank) == '-' {
                positions.push(Position::create(4, pawn_pos.file)); // 2 step pawn move
            }

            positions.push(Position::create(pawn_pos.rank - 1, pawn_pos.file)); // 1 step pawn move

            let capture_positions = get_pawn_capture_pos(&game.board, pawn_pos, 'P');

            for capture in capture_positions {
                positions.push(capture);
            }

            return positions;
        }
    }

    positions
}
pub fn get_knight_moves(board: &Board, knight_pos: &Position) -> Vec<Position> {
    let piece = get_piece_from_position(&board, &knight_pos);
    let mut positions: Vec<Position> = Vec::new();
    let rank = knight_pos.rank;
    let file = knight_pos.file;

    if piece == 'n' || piece == 'N' {
        if rank > 1 {
            if file > 0 {
                positions.push(Position::create(rank - 2, file - 1));
            }

            if file < 7 {
                positions.push(Position::create(rank - 2, file + 1));
            }
        }

        if rank < 6 {
            if file > 0 {
                positions.push(Position::create(rank + 2, file - 1));
            }

            if file < 7 {
                positions.push(Position::create(rank + 2, file + 1));
            }
        }

        if file > 1 {
            if rank > 0 {
                positions.push(Position::create(rank - 1, file - 2));
            }

            if rank < 7 {
                positions.push(Position::create(rank + 1, file - 2));
            }
        }

        if file < 6 {
            if rank > 0 {
                positions.push(Position::create(rank - 1, file + 2));
            }

            if rank < 7 {
                positions.push(Position::create(rank + 1, file + 2));
            }
        }

        let mut valid_moves = Vec::new();

        for position in positions {
            let newRank = position.rank;
            let newFile = position.file;

            if (board.get(position.rank, position.file) == '-') {
                valid_moves.push(position);
            }

            else if board.get(position.rank, position.file).is_lowercase() && board.get(knight_pos.rank, knight_pos.file).is_uppercase() {
                valid_moves.push(position);
            }

            else if board.get(position.rank, position.file).is_uppercase() && board.get(knight_pos.rank, knight_pos.file).is_lowercase() {
                valid_moves.push(position);
            }
        }

        return valid_moves;
    }

    positions
}

pub fn get_bishop_moves(board: &Board, bishop_pos: &Position) -> Vec<Position> {
    let piece = get_piece_from_position(&board, &bishop_pos);

    if piece == 'b' || piece == 'B' {
        return get_diagonal_moves(&board, &bishop_pos);
    }

    Vec::new()
}

pub fn get_queen_moves(board: &Board, queen_pos: &Position) -> Vec<Position> {
    let piece = get_piece_from_position(&board, &queen_pos);
    let mut positions = get_diagonal_moves(&board, &queen_pos);

    if piece == 'q' || piece == 'Q' {
        let horizontal_moves = get_horizontal_moves(&board, &queen_pos);
        let vertical_moves = get_vertical_moves(&board, &queen_pos);

        for h_move in horizontal_moves {
            positions.push(h_move);
        }

        for v_move in vertical_moves {
            positions.push(v_move);
        }

        return positions;
    }

    positions
}

pub fn get_king_moves(game: &Game, king_pos: &Position) -> Vec<Position> {
    let piece = get_piece_from_position(&game.board, &king_pos);

    if piece == 'k' || piece == 'K' {
        let king_rank = king_pos.rank;
        let king_file = king_pos.file;
        let mut positions:Vec<Position> = Vec::new();

        if king_rank > 0 {
            if king_file > 0 {
                positions.push(Position::create(king_rank - 1, king_file - 1));
            }

            if king_file < 7 {
                positions.push(Position::create(king_rank - 1, king_file + 1));
            }

            positions.push(Position::create(king_rank - 1, king_file ));
        }

        if king_rank < 7 {
            if king_file > 0 {
                positions.push(Position::create(king_rank + 1, king_file - 1));
            }

            if king_file < 7 {
                positions.push(Position::create(king_rank + 1, king_file + 1));
            }

            positions.push(Position::create(king_rank + 1, king_file ));
        }

        if king_file > 0 {
            positions.push(Position::create(king_rank, king_file - 1));
        }

        if king_file < 7 {
            positions.push(Position::create(king_rank, king_file + 1));
        }

        let mut valid_positions: Vec<Position>= Vec::new();
        let castle_positions = get_castling_position(game);

        for position in positions {
            if game.board.get(position.rank, position.file) == '-' {
                valid_positions.push(position)
            }
        }

        for castle_position in castle_positions {
            valid_positions.push(castle_position);
        }

        return valid_positions;
    }

    Vec::new()
}

pub fn get_rook_moves(board: &Board, rook_pos: &Position) -> Vec<Position> {
    let piece = get_piece_from_position(&board, &rook_pos);
    let mut positions = Vec::new();

    if piece == 'r' || piece == 'R' {
        let horizontal_moves = get_horizontal_moves(&board, &rook_pos);
        let vertical_moves = get_vertical_moves(&board, &rook_pos);

        for h_move in horizontal_moves {
            positions.push(h_move);
        }

        for v_move in vertical_moves {
            positions.push(v_move);
        }

        return positions;
    }

    positions
}

pub fn get_moves(game: &Game, position: &Position) -> Option<Vec<Position>> {
    let square: char = game.board.get(position.rank, position.file);

    if (square != '-') {
        let moves = match square.to_ascii_lowercase() {
            'p' => get_pawn_moves(&game, &position),
            'r' => get_rook_moves(&game.board, &position),
            'n' => get_knight_moves(&game.board, &position),
            'b' => get_bishop_moves(&game.board, &position),
            'q' => get_queen_moves(&game.board, &position),
            'k' => get_king_moves(&game, &position),
            _ => Vec::new(),
        };

        return Some(moves);
    }

    None
}

pub fn get_all_moves(game: Game) -> HashMap<Position, Vec<Position>> {
    let mut legal_moves: HashMap<Position, Vec<Position>> = HashMap::new();

    let mut row_index = 0;

    for row in &game.board.board {
        for (col, square) in row.iter().enumerate() {
            if *square != '-' {
                let piece = *square;
                let position = Position::create(row_index, col);

                let is_white = game.status == WHITE_TO_MOVE && piece.is_ascii_lowercase();
                let is_black = game.status == BLACK_TO_MOVE && piece.is_ascii_uppercase();

                if is_white || is_black {
                    let moves = get_moves(&game, &position).unwrap();

                    legal_moves.insert(Position::create(row_index, col), moves);
                }
            }
        }

        row_index += 1;
    }

    legal_moves
}

fn validate_moves(current_move: &Position, legal_moves: Vec<Position>) -> bool {
    for legal_move in legal_moves {
        if current_move.file == legal_move.file && current_move.rank == legal_move.rank {
            return true;
        }
    }

    false
}

pub fn make_move(game: &mut Game, start: &Position, end: &Position) -> Result<bool> {
    let piece: char = game.board.get(start.rank, start.file);

    if game.status == WHITE_TO_MOVE && piece.is_lowercase() {
        return Err(Error::new(ErrorKind::Other, "Error: Cannot move black piece on white turn"))
    }

    if game.status == BLACK_TO_MOVE && piece.is_uppercase() {
        return Err(Error::new(ErrorKind::Other, "Error: Cannot move white piece on black turn"))
    }

    let positions = get_moves(game, start).unwrap();

    if validate_moves(end, positions) {
        let isTargetEnemy = is_enemy(&game.board, start, end);

        game.board.board[start.rank][start.file] = '-';
        game.board.board[end.rank][end.file] = piece;

        if isTargetEnemy {
            game.current_move = CAPTURE;
        }

        else {
            game.current_move = REGULAR;
        }

        if game.status == WHITE_TO_MOVE {
            game.status = BLACK_TO_MOVE;
        }

        else if game.status == BLACK_TO_MOVE {
            game.status = WHITE_TO_MOVE;
        }

        return Ok(true);
    }

    Err(Error::new(ErrorKind::Other, "Error: Move is invalid"))
}

pub fn run() {
    let mut game = Game::new();

    loop {
        game.board.print();
        println!("");

        let mut input = String::new();

        println!("Enter position");
        stdin().read_line(&mut input).unwrap();

        let nums: Vec<usize> = input.split_whitespace().map(|w| w.parse().unwrap()).collect();
        let startRow = nums[0];
        let startFile = nums[1];

        let endRow = nums[2];
        let endFile = nums[3];

        match make_move(&mut game, &Position::create(startRow, startFile), &Position::create(endRow, endFile)) {
            Ok(v) => println!("Success"),
            Err(e) => println!("{}", e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perft_depth_1() {
        let board = Board::create();
        let nodes = board.perft(1, WHITE);
        assert_eq!(nodes, 20, "Perft Depth 1 failed: Expected 20 nodes, got {}", nodes);
    }

    #[test]
    fn test_perft_depth_2() {
        let board = Board::create();
        let nodes = board.perft(2, WHITE);
        assert_eq!(nodes, 400, "Perft Depth 2 failed: Expected 400 nodes, got {}", nodes);
    }

    #[test]
    fn test_perft_depth_3() {
        let board = Board::create();
        let nodes = board.perft(3, WHITE);
        assert_eq!(nodes, 8902, "Perft Depth 3 failed: Expected 8902 nodes, got {}", nodes);
    }
}