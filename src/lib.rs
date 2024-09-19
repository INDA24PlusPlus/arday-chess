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

pub fn add(left: usize, right: usize) -> usize {
    let boardVector = convertFENtoVector(startingFEN);

    for item in &boardVector {
        println!("{:?}", item);
    }

    println!("{:?}", &getLegalMovesForPawn(&boardVector, &Vec::from([1, 5])));

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