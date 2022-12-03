
const CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part_one(input: &str) -> usize {
    let mut priorities:Vec<usize> = Vec::new();

    for line in input.trim().split('\n') {
        let compartments = line.trim().split_at(line.len() / 2);
        
        for c in compartments.0.chars() {
            if compartments.1.contains(c) {
                let priority = CHARACTERS.chars().position(|ch| ch == c).unwrap() + 1;
                priorities.push(priority);
                break;
            }
        }
    }
    priorities.iter().sum() 
}

pub fn part_two(input: &str) -> usize {
    let mut priorities:Vec<usize> = Vec::new();
    let mut group: Vec<&str> = Vec::new();

    for (i, mut line) in input.trim().split('\n').enumerate() {
        line = line.trim();
        if i % 3 != 0 || i == 0 {
            group.push(line);
        }
        else {
            for c in group[0].chars() {
                if group[1].contains(c) && group[2].contains(c) {
                    let priority = CHARACTERS.chars().position(|ch| ch == c).unwrap() + 1;
                    priorities.push(priority);
                    break;
                }
            }

            group = vec![line];
        }
    }

    for c in group[0].chars() {
        if group[1].contains(c) && group[2].contains(c) {
            let priority = CHARACTERS.chars().position(|ch| ch == c).unwrap() + 1;
            priorities.push(priority);
            break;
        }
    }

    priorities.iter().sum() 
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_one("\n"), 0);
    // }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 7872);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 2497);
    }
}