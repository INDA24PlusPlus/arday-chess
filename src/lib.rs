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

pub fn getLegalMovesForPawn(board: &Vec<Vec<char>>, pawnPos: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let piece = getPieceFromPosition(&board, &pawnPos);

    let mut positions = Vec::new();

    if piece == 'p' {
        if pawnPos[0] == 1 {
            positions.push(Vec::from([3, pawnPos[1]]));
        }

        positions.push(Vec::from([2, pawnPos[1]]));

        return Some(positions);
    }

    else if piece == 'P' {
        if pawnPos[0] == 6 {
            positions.push(Vec::from([4, pawnPos[1]]));
        }

        positions.push(Vec::from([5, pawnPos[1]]));

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