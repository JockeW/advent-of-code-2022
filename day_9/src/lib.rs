use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut tail = (0, 0);
    visited_positions.insert((0, 0));
    let mut head = (0, 0);

    for line in input.trim().split('\n') {
        let dir: &str = line.split(' ').collect::<Vec<&str>>()[0];
        let length: &str = line.split(' ').collect::<Vec<&str>>()[1].trim();
        let length_int: i32 = length.parse::<i32>().unwrap();

        for n in 0..length_int {
            println!("n: {}", n);
            match dir {
                "U" => head = (head.0, head.1 - 1),
                "D" => head = (head.0, head.1 + 1),
                "L" => head = (head.0 - 1, head.1),
                "R" => head = (head.0 + 1, head.1),
                _ => panic!("Not a valid direction"),
            }

            //Check tail position
            if (head.1 == tail.1 && (head.0 == tail.0 + 2 || head.0 == tail.0 - 2))
                || (head.0 == tail.0 && (head.1 == tail.1 + 2 || head.1 == tail.1 - 2))
                || ((tail.0 == head.0 - 1 && tail.1 == head.1 - 2)
                    || (tail.0 == head.0 + 1 && tail.1 == head.1 - 2)
                    || (tail.0 == head.0 - 1 && tail.1 == head.1 + 2)
                    || (tail.0 == head.0 + 1 && tail.1 == head.1 + 2))
                || ((tail.1 == head.1 - 1 && tail.0 == head.0 - 2)
                    || (tail.1 == head.1 + 1 && tail.0 == head.0 - 2)
                    || (tail.1 == head.1 - 1 && tail.0 == head.0 + 2)
                    || (tail.1 == head.1 + 1 && tail.0 == head.0 + 2))
            {
                if tail.1 == head.1 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                        visited_positions.insert(tail);
                    } else {
                        tail.0 -= 1;
                        visited_positions.insert(tail);
                    }
                } else if tail.0 == head.0 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                        visited_positions.insert(tail);
                    } else {
                        tail.1 -= 1;
                        visited_positions.insert(tail);
                    }
                } else {
                    if head.1 > tail.1 {
                        if head.0 > tail.0 {
                            tail = (tail.0 + 1, tail.1 + 1);
                            visited_positions.insert(tail);
                        } else {
                            tail = (tail.0 - 1, tail.1 + 1);
                            visited_positions.insert(tail);
                        }
                    } else if head.1 < tail.1 {
                        if head.0 > tail.0 {
                            tail = (tail.0 + 1, tail.1 - 1);
                            visited_positions.insert(tail);
                        } else {
                            tail = (tail.0 - 1, tail.1 - 1);
                            visited_positions.insert(tail);
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", visited_positions);
    visited_positions.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_one(include_str!("example.txt")), 13);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 5683);
    }
}
