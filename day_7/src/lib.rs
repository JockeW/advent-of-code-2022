use std::collections::HashMap;

struct Directory {
    file_size: usize,
    child_directories: Vec<Directory>,
}

pub fn part_one(input: &str) -> usize {
    let mut file_sizes: HashMap<(&str, i32), usize> = HashMap::new();
    //file_sizes.entry("/").or_insert(0);

    let mut current_path = "";
    let mut path_stack: Vec<&str> = Vec::new();
    let mut level = 0;

    for mut line in input.trim().split('\n') {
        //println!("{:?}", file_sizes);
        line = line.trim();
        //let test = line.chars().nth(0).unwrap();
        //println!("{}", test);
        if line.chars().nth(0).unwrap() == '$' {
            let command = &line[2..4];
            //println!("COMMAND: {}", command);
            if command == "cd" {
                if &line[5..] == ".." {
                    // println!("{:?}", path_stack);
                    let parent_path = path_stack[path_stack.len() - 2];
                    let current_path_file_size = file_sizes
                        .entry((current_path, level))
                        .or_default()
                        .to_owned();

                    path_stack.pop();
                    level -= 1;
                    file_sizes
                        .entry((parent_path, level))
                        .and_modify(|size| *size += current_path_file_size)
                        .or_insert(current_path_file_size);
                    current_path = parent_path;
                } else {
                    //println!("Go to dir");
                    current_path += &line[5..];
                    path_stack.push(current_path);
                    level += 1;
                }
            }
        } else if !line.starts_with("dir") {
            //println!("{}", current_path);
            let file_size: usize = line.trim().split(' ').next().unwrap().parse().unwrap();
            file_sizes
                .entry((current_path, level))
                .and_modify(|size| *size += file_size)
                .or_insert(file_size);
        }
    }

    // println!("{:?}", file_sizes);
    let smallest_directories: Vec<&usize> = file_sizes
        .iter()
        .filter(|size| *size.1 <= 100000)
        .map(|size| size.1)
        .collect();


    smallest_directories.into_iter().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_one(include_str!("example.txt")), 95437);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_one(include_str!("example2.txt")), 3);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 0);
    }
}
