pub fn part_one(input: &str) -> i64 {
    for line in input.trim().split('\n') {
        //
    }
    0
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_one("\n"), 0);
    // }

    #[test]
    fn part_one() {
        assert_eq!(super::part_a(include_str!("input.txt")), 0);
    }
}