use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let line = lines.first().unwrap();

    let mut chars: Vec<char> = Vec::with_capacity(4);
    for (i, c) in line.chars().enumerate() {
        if chars.len() == 4 {
            let test = chars.iter().collect::<HashSet<&char>>();
            if test.len() == 4 {
                return i;
            }
            else {
                chars.remove(0);
            }
        }

        chars.push(c);
    }

    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_one(include_str!("example.txt")), 11);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 1794);
    }
}