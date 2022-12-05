pub fn part_one(input: &str) -> i64 {
    let mut lines: Vec<&str> = input.split('\n').collect();
    //lines = lines.iter().map(|l| l.trim()).collect();

    let setup_lines = lines[0..lines.iter().position(|&line| line.chars().nth(1).unwrap() == '1').unwrap()].to_vec();
    println!("{:?}", setup_lines);
    let number_of_stacks: u32 = setup_lines.last().unwrap().chars().last().unwrap().to_digit(10).unwrap();
    //println!("{}", number_of_stacks);

    
    for line in setup_lines {
        println!("{}", line);
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
        assert_eq!(super::part_one(include_str!("input.txt")), 2);
    }
}