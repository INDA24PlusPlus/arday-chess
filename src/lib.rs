const startingFEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn convertFENtoVector(fen: &str) -> Vec<Vec<char>> {
    let mut newVector = Vec::new();
    let position = startingFEN.split(" ").collect::<Vec<&str>>()[0];

    for rank in position.split("/").collect::<Vec<&str>>() {
        let mut rowVector = Vec::new();

        for square in rank.chars() {
            match square.to_digit(10) {
                Some(num) => {
                    let num = num as i32;

                    for i in 0..num {
                        rowVector.push('-');
                    }
                }

                None => rowVector.push(square)
            }
        }

        newVector.push(rowVector);
    }

    newVector
}

pub fn getPieceFromPosition(board: &Vec<Vec<char>>, piecePos: &Vec<usize>) -> char {
    board[piecePos[0]][piecePos[1]]
}

pub fn hasEnemyPiece(board: &Vec<Vec<char>>, pos: &Vec<usize>, currentPiece: char) -> bool {
    let piece = getPieceFromPosition(&board, &pos);

    if currentPiece.is_uppercase() && piece.is_lowercase() {
        return true;
    }

    if currentPiece.is_lowercase() && piece.is_uppercase() {
        return true;
    }

    false
}

pub fn getPawnCapturePos(board: &Vec<Vec<char>>, pawnPos: &Vec<usize>, pawn: char) -> Option<Vec<Vec<usize>>> {
    let mut target = Vec::new();
    let mut targetRank = 0;

    if (pawn.is_lowercase()) {
        targetRank = pawnPos[0] + 1;
    }

    else {
        targetRank = pawnPos[0] - 1;
    }

    let square1 = pawnPos[1] + 1;
    let square2 = pawnPos[1] - 1;

    let target1 = Vec::from([targetRank, square1]);
    let target2 = Vec::from([targetRank, square2]);

    let hasEnemy1 = hasEnemyPiece(&board, &target1, pawn);
    let hasEnemy2 = hasEnemyPiece(&board, &target2, pawn);

    if pawnPos[1] == 0 && hasEnemy1 {
        target.push(Vec::from([targetRank, square1]));
    }

    else if pawnPos[1] == 7 && hasEnemy2 {
        target.push(Vec::from([targetRank, square2]));
    }

    else {
        if hasEnemy1 {
            target.push(Vec::from([targetRank, square1]));
        }

        if hasEnemy2 {
            target.push(Vec::from([targetRank, square2]));
        }
    }

    Some(target)
}

pub fn getLegalMovesForPawn(board: &Vec<Vec<char>>, pawnPos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let piece = getPieceFromPosition(&board, &pawnPos);

    let mut positions = Vec::new();

    if piece == 'p' {
        if pawnPos[0] == 1 {
            positions.push(Vec::from([3, pawnPos[1]])); // 2 step pawn move
        }

        positions.push(Vec::from([2, pawnPos[1]])); // 1 step pawn move

        return Some(positions);
    }

    else if piece == 'P' {
        if pawnPos[0] == 6 {
            positions.push(Vec::from([4, pawnPos[1]])); // 2 step pawn move
        }

        positions.push(Vec::from([5, pawnPos[1]])); // 1 step pawn move

        return Some(positions);
    }

    println!("No pawn found");
    None
}

pub fn getLegalMovesForKnight(board: &Vec<Vec<char>>, knightPos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let piece = getPieceFromPosition(&board, &knightPos);
    let mut positions = Vec::new();
    let rank = knightPos[0];
    let file = knightPos[1];

    if piece == 'n' || piece == 'N' {
        if (rank > 1) {
            if (file > 0) {
                positions.push(Vec::from([rank - 2, file - 1]));
            }

            if (file < 7) {
                positions.push(Vec::from([rank - 2, file + 1]));
            }
        }

        if (rank < 6) {
            if (file > 0) {
                positions.push(Vec::from([rank + 2, file - 1]));
            }

            if (file < 7) {
                positions.push(Vec::from([rank + 2, file + 1]));
            }
        }

        if (file > 1) {
            if (rank > 0) {
                positions.push(Vec::from([rank - 1, file - 2]));
            }

            if (rank < 7) {
                positions.push(Vec::from([rank + 1, file - 2]));
            }
        }

        if (file < 6) {
            if (rank > 0) {
                positions.push(Vec::from([rank - 1, file + 2]));
            }

            if (rank < 7) {
                positions.push(Vec::from([rank + 1, file + 2]));
            }
        }

        return Some(positions);
    }

    None
}

fn getHorizontalMoves(board: &Vec<Vec<char>>, piecePos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let mut positions = Vec::new();
    let rookRank = piecePos[0];
    let rookFile = piecePos[1];

    for file in 0..8 {
        if (file == rookFile) {
            continue;
        }

        positions.push(Vec::from([rookRank, file]));
    }

    Some(positions)
}

fn getVerticalMoves(board: &Vec<Vec<char>>, piecePos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let mut positions = Vec::new();
    let rookRank = piecePos[0];
    let rookFile = piecePos[1];

    for rank in 0..8 {
        if (rank == rookRank) {
            continue;
        }

        positions.push(Vec::from([rank, rookFile]));
    }

    Some(positions)
}

pub fn getLegalMovesForRook(board: &Vec<Vec<char>>, rookPos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let piece = getPieceFromPosition(&board, &rookPos);
    let mut positions = Vec::new();

    if (piece == 'r' || piece == 'R') {
        let horizontalMoves = getHorizontalMoves(&board, &rookPos).unwrap();
        let verticalMoves = getVerticalMoves(&board, &rookPos).unwrap();

        for h_move in horizontalMoves {
            positions.push(h_move);
        }

        for v_move in verticalMoves {
            positions.push(v_move);
        }

        return Some(positions);
    }

    None
}

pub fn add(left: usize, right: usize) -> usize {
    let boardVector = convertFENtoVector(startingFEN);

    for item in &boardVector {
        println!("{:?}", item);
    }

    println!("{:?}", &getLegalMovesForRook(&boardVector, &Vec::from([0, 7])));

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