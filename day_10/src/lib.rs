pub fn part_one(input: &str) -> i32 {
    let mut signal_strengths: Vec<i32> = Vec::new();
    let mut x = 1;
    let mut cycle = 0;

    for line in input.trim().split('\n') {
        if cycle % 40 == 20 {
            signal_strengths.push(cycle * x);

            if cycle == 220 {
                break;
            }
        }
        let instruction: &str = line.split(' ').collect::<Vec<&str>>()[0].trim();

        match instruction {
            "noop" => {
                cycle += 1;
            }
            "addx" => {
                let value_str: &str = line.split(' ').collect::<Vec<&str>>()[1].trim();
                let value: i32 = value_str.parse::<i32>().unwrap();
                cycle += 1;
                if cycle % 40 == 20 {
                    signal_strengths.push(cycle * x);

                    if cycle == 220 {
                        break;
                    }
                }
                cycle += 1;
                if cycle % 40 == 20 {
                    signal_strengths.push(cycle * x);

                    if cycle == 220 {
                        break;
                    }
                }

                x += value;
            }
            _ => panic!("Not a valid instruction"),
        }
    }

    signal_strengths.iter().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_one(include_str!("example.txt")), 13140);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 18120);
    }
}
