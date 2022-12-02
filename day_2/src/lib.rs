#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors
}

pub fn part_one(input: &str) -> u32 {
    let mut score: u32 = 0;
    for line in input.trim().split('\n') {
        let round: Vec<Move> = line
            .trim()
            .split(' ')
            .map(|c| to_move(c))
            .collect();

        score += calculate_round_score(round);
    }
    
    score
}

pub fn part_two(input: &str) -> u32 {
    let mut score: u32 = 0;
    for line in input.trim().split('\n') {
        let round: Vec<Move> = line
            .trim()
            .split(' ')
            .map(|c| to_move(c))
            .collect();

        let new_moves = get_moves(round);

        score += calculate_round_score(new_moves);
    }
    
    score
}

fn to_move(c: &str) -> Move {
    match c {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!("Invalid character detected!")
    }
}

fn calculate_round_score(round: Vec<Move>) -> u32 {
    match round[..] {
        [Move::Rock, Move::Rock] => 3 + 1,
        [Move::Rock, Move::Paper] => 6 + 2,
        [Move::Rock, Move::Scissors] => 0 + 3,
        [Move::Paper, Move::Rock] => 0 + 1,
        [Move::Paper, Move::Paper] => 3 + 2,
        [Move::Paper, Move::Scissors] => 6 + 3,
        [Move::Scissors, Move::Rock] => 6 + 1,
        [Move::Scissors, Move::Paper] => 0 + 2,
        [Move::Scissors, Move::Scissors] => 3 + 3,
        _ => panic!()
    }
}

fn get_moves(round: Vec<Move>) -> Vec<Move> {
    match round[..] {
        [Move::Rock, Move::Rock] => vec![Move::Rock, Move::Scissors],
        [Move::Rock, Move::Paper] =>vec![Move::Rock, Move::Rock],
        [Move::Rock, Move::Scissors] => vec![Move::Rock, Move::Paper],
        [Move::Paper, Move::Rock] => vec![Move::Paper, Move::Rock],
        [Move::Paper, Move::Paper] => vec![Move::Paper, Move::Paper],
        [Move::Paper, Move::Scissors] => vec![Move::Paper, Move::Scissors],
        [Move::Scissors, Move::Rock] => vec![Move::Scissors, Move::Paper],
        [Move::Scissors, Move::Paper] => vec![Move::Scissors, Move::Scissors],
        [Move::Scissors, Move::Scissors] => vec![Move::Scissors, Move::Rock],
        _ => panic!()
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_one("example.txt"), 0);
    // }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 8890);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 10238);
    }
}