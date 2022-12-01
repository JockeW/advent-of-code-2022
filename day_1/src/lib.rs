pub fn part_one(input: &str) -> u32 {
    let mut max = 0;
    let mut current_elf:u32 = 0;
    for line in input.trim().split('\n') {
        let line_value: Result<u32, _> = line.trim().parse();
        println!("{:?}", line_value);
        if line_value.is_err() {
            if current_elf > max {
                max = current_elf;
            }
            current_elf = 0;
            continue;
        }

        current_elf += line_value.unwrap();
    }
    max
}

pub fn part_two(input: &str) -> u32 {
    let mut elfs: Vec<u32> = Vec::new();
    let mut current_elf:u32 = 0;

    for line in input.trim().split('\n') {
        let line_value: Result<u32, _> = line.trim().parse();
        if line_value.is_err() {
            elfs.push(current_elf);
            current_elf = 0;
            continue;
        }
        
        current_elf += line_value.unwrap();
    }

    elfs.sort_by(|a, b| b.cmp(a));
    
    elfs[0] + elfs[1] + elfs[2]
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_one(include_str!("example.txt")), 24000);
    // }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 71780);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 212489);
    }

}