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

pub fn add(left: usize, right: usize) -> usize {
    let boardVector = convertFENtoVector(startingFEN);

    for item in boardVector {
        println!("{:?}", item);
    }

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