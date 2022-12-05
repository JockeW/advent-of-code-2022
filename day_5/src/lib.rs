
pub fn part_one(input: &str) -> String {
    let mut lines: Vec<&str> = input.lines().collect();
    //lines = lines.iter().map(|l| l.trim()).collect();
    //println!("{:?}", lines);
    let blank_line_index = lines.iter().position(|&l| l == "").unwrap();

    let mut setup_lines = lines[0..blank_line_index].to_vec();
    //println!("{:?}", setup_lines);
    //let stack_number_line = setup_lines.last().unwrap();
    let stack_number_line = setup_lines.remove(setup_lines.len() - 1);
    //println!("{:?}", stack_number_line);
    let number_of_stacks: u32 = stack_number_line.chars().nth(stack_number_line.len() - 2).unwrap().to_digit(10).unwrap();
    //println!("{}", number_of_stacks);

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(number_of_stacks.try_into().unwrap());
    //println!("{:?}", stacks);

    for line in setup_lines {
        let mut count = 0;
        for i in (1..stack_number_line.len()).step_by(4) {
            let line_char = line.chars().nth(i).unwrap();
            //println!("{}", line_char);
            if stacks.len() > count {
                stacks[count].push(line_char);
            }
            else {
                stacks.push(vec![line_char]);
            }
            count += 1;
        }
    }

    let mut reversed_stacks: Vec<Vec<char>> = Vec::with_capacity(number_of_stacks.try_into().unwrap());
    for mut stack in stacks {
        stack.retain(|&c| c != ' ');
        stack.reverse();
        reversed_stacks.push(stack.clone());
        //println!("{:?}", stack);
    }
    //reversed_stacks.iter().filter(predicate)
    println!("{:?}", reversed_stacks);

    let instruction_lines = lines[blank_line_index + 1..lines.len()].to_vec();
    //println!("{:?}", instruction_lines);

    for instruction in instruction_lines {
        let instruction_parts: Vec<&str> = instruction.trim().split(' ').collect();
        let crates: usize = instruction_parts[1].parse().unwrap();
        let from: usize = instruction_parts[3].parse().unwrap();
        let to: usize = instruction_parts[5].parse().unwrap();
        println!("{} {} {}", crates, from, to);

        for _i in 0..crates {
            //let idx = i as usize;
            let from_char = reversed_stacks[from-1].pop().unwrap();
            //println!("{}", from_char);

            reversed_stacks[to-1].push(from_char);
        }
    }

    let mut answer: String = String::new();
    for stack in reversed_stacks {
        //println!("{:?}", stack);
        answer += &stack.last().unwrap().to_string();
    }
    
    
    answer
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_one(include_str!("example.txt")), "CMZ");
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), "HBTMTBSDC");
    }
}