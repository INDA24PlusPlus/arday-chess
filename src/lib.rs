const STARTING_FEN: &str = "r4k1r/2pbnppp/4p3/p1Ppb3/3P4/1Pp1P1P1/P1n2PBP/1R2R1K1";

pub fn convert_fen_to_vector(fen: &str) -> Vec<Vec<char>> {
    let mut new_vector = Vec::new();
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

        new_vector.push(row_vector);
    }

    new_vector
}

pub fn get_piece_from_position(board: &Vec<Vec<char>>, piece_pos: &Vec<usize>) -> char {
    board[piece_pos[0]][piece_pos[1]]
}

pub fn has_enemy_piece(board: &Vec<Vec<char>>, pos: &Vec<usize>, current_piece: char) -> bool {
    let piece = get_piece_from_position(&board, &pos);

    if current_piece.is_uppercase() && piece.is_lowercase() {
        return true;
    }

    if current_piece.is_lowercase() && piece.is_uppercase() {
        return true;
    }

    false
}

pub fn get_pawn_capture_pos(board: &Vec<Vec<char>>, pawn_pos: &Vec<usize>, pawn: char) -> Vec<Vec<usize>> {
    let mut target = Vec::new();
    let mut target_rank = 0;

    if pawn.is_lowercase() {
        target_rank = pawn_pos[0] + 1;
    }

    else {
        target_rank = pawn_pos[0] - 1;
    }

    if (pawn_pos[1] > 0) {
        let square2 = pawn_pos[1] - 1;
        let target2 = Vec::from([target_rank, square2]);
        let has_enemy2 = has_enemy_piece(&board, &target2, pawn);

        if pawn_pos[1] == 7 && has_enemy2 {
            target.push(Vec::from([target_rank, square2]));
        }

        else if has_enemy2 {
            target.push(Vec::from([target_rank, square2]));
        }
    }

    if (pawn_pos[1] < 7) {
        let square1 = pawn_pos[1] + 1;
        let target1 = Vec::from([target_rank, square1]);
        let has_enemy1 = has_enemy_piece(&board, &target1, pawn);

        if pawn_pos[1] == 0 && has_enemy1 {
            target.push(Vec::from([target_rank, square1]));
        }

        else if has_enemy1 {
            target.push(Vec::from([target_rank, square1]));
        }
    }

    target
}

pub fn get_legal_moves_for_pawn(board: &Vec<Vec<char>>, pawn_pos: &Vec<usize>) -> Vec<Vec<usize>> {
    let piece = get_piece_from_position(&board, &pawn_pos);

    let mut positions = Vec::new();

    if piece == 'p' {
        if (board[pawn_pos[0] + 1][pawn_pos[1]] == '-') {
            if pawn_pos[0] == 1 && board[3][pawn_pos[1]] == '-' {
                positions.push(Vec::from([3, pawn_pos[1]])); // 2 step pawn move
            }

            positions.push(Vec::from([pawn_pos[0] + 1, pawn_pos[1]])); // 1 step pawn move

            return positions;
        }
    } else if piece == 'P' {
        if board[pawn_pos[0] - 1][pawn_pos[1]] == '-' {
            if pawn_pos[0] == 6 && board[4][pawn_pos[1]] == '-' {
                positions.push(Vec::from([4, pawn_pos[1]])); // 2 step pawn move
            }

            positions.push(Vec::from([pawn_pos[0] - 1, pawn_pos[1]])); // 1 step pawn move

            return positions;
        }
    }

    println!("No pawn found");
    positions
}

pub fn get_legal_moves_for_knight(board: &Vec<Vec<char>>, knight_pos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let piece = get_piece_from_position(&board, &knight_pos);
    let mut positions = Vec::new();
    let rank = knight_pos[0];
    let file = knight_pos[1];

    if piece == 'n' || piece == 'N' {
        if rank > 1 {
            if file > 0 {
                positions.push(Vec::from([rank - 2, file - 1]));
            }

            if file < 7 {
                positions.push(Vec::from([rank - 2, file + 1]));
            }
        }

        if rank < 6 {
            if file > 0 {
                positions.push(Vec::from([rank + 2, file - 1]));
            }

            if file < 7 {
                positions.push(Vec::from([rank + 2, file + 1]));
            }
        }

        if file > 1 {
            if rank > 0 {
                positions.push(Vec::from([rank - 1, file - 2]));
            }

            if rank < 7 {
                positions.push(Vec::from([rank + 1, file - 2]));
            }
        }

        if file < 6 {
            if rank > 0 {
                positions.push(Vec::from([rank - 1, file + 2]));
            }

            if rank < 7 {
                positions.push(Vec::from([rank + 1, file + 2]));
            }
        }

        return Some(positions);
    }

    None
}

fn get_horizontal_moves(board: &Vec<Vec<char>>, piece_pos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let mut positions = Vec::new();
    let rank = piece_pos[0];
    let mut file = piece_pos[1];

    loop {
        if file == 7 {
            if rank != piece_pos[0] || file != piece_pos[1] {
                positions.push(Vec::from([rank, 7]));
            }

            file = piece_pos[1];

            break;
        }

        if rank == piece_pos[0] && file == piece_pos[1] {
            file += 1;
            continue;
        }

        if board[rank][file] != '-'  {
            file = piece_pos[1];

            break;
        }

        positions.push(Vec::from([rank, file]));

        file += 1;
    }

    loop {
        if file == 0 {
            println!("Adding {}, {}", rank, file);
            if rank != piece_pos[0] || file != piece_pos[1] {
                positions.push(Vec::from([rank, 0]));
            }

            file = piece_pos[1];

            break;
        }

        if rank == piece_pos[0] && file == piece_pos[1] {
            file -= 1;
            continue;
        }

        if board[rank][file] != '-' {
            file = piece_pos[1];

            break;
        }

        positions.push(Vec::from([rank, file]));

        file -= 1;
    }

    Some(positions)
}

fn get_vertical_moves(board: &Vec<Vec<char>>, piece_pos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let mut positions = Vec::new();
    let mut rank = piece_pos[0];
    let file = piece_pos[1];

    loop {
        if rank == 7 {
            if file != piece_pos[1] || rank != piece_pos[0] {
                positions.push(Vec::from([7, file]));
            }

            rank = piece_pos[0];

            break;
        }

        if rank == piece_pos[0] && file == piece_pos[1] {
            rank += 1;
            continue;
        }

        if board[rank][file] != '-' {
            rank = piece_pos[0];

            break;
        }

        positions.push(Vec::from([0, file]));

        rank += 1;
    }

    loop {
        if rank == 0 {
            if file != piece_pos[1] || rank != piece_pos[0] {
                positions.push(Vec::from([rank, file]));
            }

            rank = piece_pos[0];

            break;
        }

        if rank == piece_pos[0] && file == piece_pos[1] {
            rank -= 1;
            continue;
        }

        if board[rank][file] != '-' {
            rank = piece_pos[0];

            break;
        }

        positions.push(Vec::from([rank, file]));

        rank -= 1;
    }

    Some(positions)
}

pub fn get_legal_moves_for_rook(board: &Vec<Vec<char>>, rook_pos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let piece = get_piece_from_position(&board, &rook_pos);
    let mut positions = Vec::new();

    if piece == 'r' || piece == 'R' {
        let horizontal_moves = get_horizontal_moves(&board, &rook_pos)?;
        let vertical_moves = get_vertical_moves(&board, &rook_pos)?;

        for h_move in horizontal_moves {
            positions.push(h_move);
        }

        for v_move in vertical_moves {
            positions.push(v_move);
        }

        return Some(positions);
    }

    None
}

fn get_diagonal_moves(board: &Vec<Vec<char>>, piece_pos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let mut positions = Vec::new();
    let mut rank = piece_pos[0];
    let mut file = piece_pos[1];

    if rank > 0 && file > 0 {
        loop {
            if board[rank - 1][file - 1] != '-' {
                rank = piece_pos[0];
                file = piece_pos[1];
                break;
            }

            positions.push(Vec::from([rank - 1, file - 1]));

            rank = rank - 1;
            file = file - 1;

            if rank == 0 || file == 0 {
                rank = piece_pos[0];
                file = piece_pos[1];
                break;
            }
        }
    }

    if rank > 0 && file < 7 {
        loop {
            if board[rank - 1][file + 1] != '-' {
                rank = piece_pos[0];
                file = piece_pos[1];
                break;
            }

            positions.push(Vec::from([rank - 1, file + 1]));

            rank = rank - 1;
            file = file + 1;

            if rank == 0 || file == 7 {
                rank = piece_pos[0];
                file = piece_pos[1];
                break;
            }
        }
    }

    if rank < 7 && file > 0 {
        loop {
            if board[rank + 1][file - 1] != '-' {
                rank = piece_pos[0];
                file = piece_pos[1];
                break;
            }

            positions.push(Vec::from([rank + 1, file - 1]));

            rank = rank + 1;
            file = file - 1;

            if rank == 7 || file == 0 {
                rank = piece_pos[0];
                file = piece_pos[1];
                break;
            }
        }
    }

    if rank < 7 && file < 7 {
        loop {
            if board[rank + 1][file + 1] != '-' {
                rank = piece_pos[0];
                file = piece_pos[1];
                break;
            }

            positions.push(Vec::from([rank + 1, file + 1]));

            rank = rank + 1;
            file = file + 1;

            if rank == 7 || file == 7 {
                rank = piece_pos[0];
                file = piece_pos[1];
                break;
            }
        }
    }

    Some(positions)
}

pub fn get_legal_moves_for_bishop(board: &Vec<Vec<char>>, bishop_pos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let piece = get_piece_from_position(&board, &bishop_pos);

    if piece == 'b' || piece == 'B' {
        return Some(get_diagonal_moves(&board, &bishop_pos)?);
    }

    None
}

pub fn get_legal_moves_for_queen(board: &Vec<Vec<char>>, queen_pos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let piece = get_piece_from_position(&board, &queen_pos);
    let mut positions = get_diagonal_moves(&board, &queen_pos)?;

    if piece == 'q' || piece == 'Q' {
        let horizontal_moves = get_horizontal_moves(&board, &queen_pos)?;
        let vertical_moves = get_vertical_moves(&board, &queen_pos)?;

        for h_move in horizontal_moves {
            positions.push(h_move);
        }

        for v_move in vertical_moves {
            positions.push(v_move);
        }

        return Some(positions);
    }

    None
}

pub fn get_legal_moves_for_king(board: &Vec<Vec<char>>, king_pos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let piece = get_piece_from_position(&board, &king_pos);

    if piece == 'k' || piece == 'K' {
        let king_rank = king_pos[0];
        let king_file = king_pos[1];
        let mut positions = Vec::new();

        if king_rank > 0 {
            if king_file > 0 {
                positions.push(Vec::from([king_rank - 1, king_file - 1]));
            }

            if king_file < 7 {
                positions.push(Vec::from([king_rank - 1, king_file + 1]));
            }

            positions.push(Vec::from([king_rank - 1, king_file]));
        }

        if king_rank < 7 {
            if king_file > 0 {
                positions.push(Vec::from([king_rank + 1, king_file - 1]));
            }

            if king_file < 7 {
                positions.push(Vec::from([king_rank + 1, king_file + 1]));
            }

            positions.push(Vec::from([king_rank + 1, king_file]));
        }

        if king_file > 0 {
            positions.push(Vec::from([king_rank, king_file - 1]));
        }

        if king_file < 7 {
            positions.push(Vec::from([king_rank, king_file + 1]));
        }

        let mut valid_positions= Vec::new();

        for position in positions {
            if board[position[0]][position[1]] == '-' {
                valid_positions.push(Vec::from(position))
            }
        }

        return Some(valid_positions);
    }

    None
}

pub fn add(left: usize, right: usize) -> usize {
    let board_vector = convert_fen_to_vector(STARTING_FEN);

    for item in &board_vector {
        println!("{:?}", item);
    }

    println!("{:?}", &get_legal_moves_for_pawn(&board_vector, &Vec::from([4, 3])));

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}