
pub fn part_one(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.trim().split('\n') {
        let ranges: Vec<&str> = line.trim().split(',').collect();

        let first_range: Vec<u32> = ranges[0].split('-').map(|s| s.parse::<u32>().unwrap()).collect(); 
        let second_range: Vec<u32> = ranges[1].split('-').map(|s| s.parse::<u32>().unwrap()).collect(); 

        if (first_range[0] >= second_range[0] && first_range[1] <= second_range[1]) 
            || (second_range[0] >= first_range[0] && second_range[1] <= first_range[1]) {
                total += 1;
        }
    }

    total
}

pub fn part_two(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.trim().split('\n') {
        let ranges: Vec<&str> = line.trim().split(',').collect();

        let first_range: Vec<u32> = ranges[0].split('-').map(|s| s.parse::<u32>().unwrap()).collect(); 
        let second_range: Vec<u32> = ranges[1].split('-').map(|s| s.parse::<u32>().unwrap()).collect(); 

        if first_range[0] <= second_range[1] && second_range[0] <= first_range[1] {
                total += 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_one(include_str!("example.txt")), 2);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 459);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 779);
    }
}