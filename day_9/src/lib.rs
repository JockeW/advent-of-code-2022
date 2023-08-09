use std::collections::HashSet;

fn update_positions(head: (i32, i32), mut tail: (i32, i32)) -> (i32, i32) {
    if (head.0 == tail.0 + 2 || head.0 == tail.0 - 2)
        && (head.1 == tail.1 + 2 || head.1 == tail.1 - 2)
    {
        if head.0 == tail.0 + 2 {
            tail.0 += 1;
        } else {
            tail.0 -= 1;
        }

        if head.1 == tail.1 + 2 {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }
    } else if (head.1 == tail.1 && (head.0 == tail.0 + 2 || head.0 == tail.0 - 2))
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
            } else {
                tail.0 -= 1;
            }
        } else if tail.0 == head.0 {
            if head.1 > tail.1 {
                tail.1 += 1;
            } else {
                tail.1 -= 1;
            }
        } else {
            if head.1 > tail.1 {
                if head.0 > tail.0 {
                    tail = (tail.0 + 1, tail.1 + 1);
                } else {
                    tail = (tail.0 - 1, tail.1 + 1);
                }
            } else if head.1 < tail.1 {
                if head.0 > tail.0 {
                    tail = (tail.0 + 1, tail.1 - 1);
                } else {
                    tail = (tail.0 - 1, tail.1 - 1);
                }
            }
        }
    }

    tail
}

pub fn part_one(input: &str) -> usize {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut tail = (0, 0);
    visited_positions.insert((0, 0));
    let mut head = (0, 0);

    for line in input.trim().split('\n') {
        let dir: &str = line.split(' ').collect::<Vec<&str>>()[0];
        let length: &str = line.split(' ').collect::<Vec<&str>>()[1].trim();
        let length_int: i32 = length.parse::<i32>().unwrap();

        for _ in 0..length_int {
            match dir {
                "U" => head = (head.0, head.1 - 1),
                "D" => head = (head.0, head.1 + 1),
                "L" => head = (head.0 - 1, head.1),
                "R" => head = (head.0 + 1, head.1),
                _ => panic!("Not a valid direction"),
            }

            tail = update_positions(head, tail);
            visited_positions.insert(tail);
        }
    }

    println!("{:?}", visited_positions);
    visited_positions.len()
}

fn print_rope(width: i32, height: i32, knots: [(i32, i32); 10]) {
    for y in 0..height {
        for x in 0..width {
            if knots.contains(&(x, y)) {
                let knot_index = knots.iter().position(|&k| k.0 == x && k.1 == y).unwrap();
                println!("{}", knot_index);
            } else {
                println!("*")
            }
        }
    }
}

pub fn part_two(input: &str) -> usize {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut knots: [(i32, i32); 10] = [(0, 0); 10];

    for line in input.trim().split('\n') {
        let dir: &str = line.split(' ').collect::<Vec<&str>>()[0];
        let length: &str = line.split(' ').collect::<Vec<&str>>()[1].trim();
        let length_int: i32 = length.parse::<i32>().unwrap();

        for _ in 0..length_int {
            match dir {
                "U" => knots[0] = (knots[0].0, knots[0].1 - 1),
                "D" => knots[0] = (knots[0].0, knots[0].1 + 1),
                "L" => knots[0] = (knots[0].0 - 1, knots[0].1),
                "R" => knots[0] = (knots[0].0 + 1, knots[0].1),
                _ => panic!("Not a valid direction"),
            }

            for i in 0..knots.len() {
                if i == knots.len() - 1 {
                    break;
                }
                knots[i + 1] = update_positions(knots[i], knots[i + 1]);
            }

            visited_positions.insert(*knots.last().unwrap());
        }
    }

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

    #[test]
    fn example2() {
        assert_eq!(super::part_two(include_str!("example2.txt")), 36);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 2372);
    }
}
