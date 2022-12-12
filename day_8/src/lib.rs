#[derive(Debug)]
struct Tree {
    height: usize,
    x: usize,
    y: usize,
}

impl Tree {
    fn new(height: usize, x: usize, y: usize) -> Tree {
        Tree { height, x, y }
    }
}

pub fn part_one(input: &str) -> usize {
    let trees = get_trees(input);

    let max_x = trees
        .iter()
        .max_by(|&a, &b| a.x.cmp(&b.x))
        .map(|tree| tree.x)
        .unwrap();
    let max_y = trees
        .iter()
        .max_by(|&a, &b| a.y.cmp(&b.y))
        .map(|tree| tree.y)
        .unwrap();

    let interior_trees = trees
        .iter()
        .filter(|&tree| tree.x != 0 && tree.x != max_x && tree.y != 0 && tree.y != max_y)
        .collect::<Vec<&Tree>>();

    let edge_trees = trees
        .iter()
        .filter(|&tree| tree.x == 0 || tree.x == max_x || tree.y == 0 || tree.y == max_y)
        .collect::<Vec<&Tree>>();

    let mut visible_tree_count = edge_trees.iter().count();

    for interior_tree in interior_trees {
        let visible_left = trees
            .iter()
            .filter(|&tree| tree.x < interior_tree.x && tree.y == interior_tree.y)
            .all(|tree| tree.height < interior_tree.height);

        let visible_right = trees
            .iter()
            .filter(|&tree| tree.x > interior_tree.x && tree.y == interior_tree.y)
            .all(|tree| tree.height < interior_tree.height);

        let visible_top = trees
            .iter()
            .filter(|&tree| tree.x == interior_tree.x && tree.y < interior_tree.y)
            .all(|tree| tree.height < interior_tree.height);

        let visible_bottom = trees
            .iter()
            .filter(|&tree| tree.x == interior_tree.x && tree.y > interior_tree.y)
            .all(|tree| tree.height < interior_tree.height);

        if visible_left || visible_right || visible_top || visible_bottom {
            visible_tree_count += 1;
        }
    }

    visible_tree_count
}

pub fn part_two(input: &str) -> usize {
    let trees = get_trees(input);

    let max_x = trees
        .iter()
        .max_by(|&a, &b| a.x.cmp(&b.x))
        .map(|tree| tree.x)
        .unwrap();
    let max_y = trees
        .iter()
        .max_by(|&a, &b| a.y.cmp(&b.y))
        .map(|tree| tree.y)
        .unwrap();

    let interior_trees = trees
        .iter()
        .filter(|&tree| tree.x != 0 && tree.x != max_x && tree.y != 0 && tree.y != max_y)
        .collect::<Vec<&Tree>>();

    let mut scenic_scores = Vec::new();

    for interior_tree in interior_trees {
        scenic_scores.push(get_scenic_score(interior_tree, &trees));
    }

    *scenic_scores.iter().max().unwrap()
}

fn get_scenic_score(tree: &Tree, all_trees: &Vec<Tree>) -> usize {
    let mut trees_above = all_trees
        .iter()
        .filter(|&t| t.y < tree.y && t.x == tree.x)
        .collect::<Vec<&Tree>>();

    trees_above.sort_by(|&a, &b| b.y.cmp(&a.y));

    let mut above = 0;
    for (idx, t) in trees_above.iter().enumerate() {
        if t.height >= tree.height || idx == trees_above.len() - 1 {
            above = idx + 1;
            break;
        }
    }

    let mut trees_below = all_trees
        .iter()
        .filter(|&t| t.y > tree.y && t.x == tree.x)
        .collect::<Vec<&Tree>>();

    trees_below.sort_by(|&a, &b| a.y.cmp(&b.y));

    let mut below = 0;
    for (idx, t) in trees_below.iter().enumerate() {
        if t.height >= tree.height || idx == trees_below.len() - 1 {
            below = idx + 1;
            break;
        }
    }

    let mut trees_left = all_trees
        .iter()
        .filter(|&t| t.y == tree.y && t.x < tree.x)
        .collect::<Vec<&Tree>>();

    trees_left.sort_by(|&a, &b| b.x.cmp(&a.x));

    let mut left = 0;
    for (idx, t) in trees_left.iter().enumerate() {
        if t.height >= tree.height || idx == trees_left.len() - 1 {
            left = idx + 1;
            break;
        }
    }

    let mut trees_right = all_trees
        .iter()
        .filter(|&t| t.y == tree.y && t.x > tree.x)
        .collect::<Vec<&Tree>>();

    trees_right.sort_by(|&a, &b| a.x.cmp(&b.x));

    let mut right = 0;
    for (idx, t) in trees_right.iter().enumerate() {
        if t.height >= tree.height || idx == trees_right.len() - 1 {
            right = idx + 1;
            break;
        }
    }

    above * below * left * right
}

fn get_trees(input: &str) -> Vec<Tree> {
    let mut trees = Vec::new();
    for (y, mut line) in input.trim().split('\n').enumerate() {
        line = line.trim();
        for (x, height_char) in line.chars().enumerate() {
            let height = height_char.to_digit(10).unwrap();
            trees.push(Tree::new(height.try_into().unwrap(), x, y));
        }
    }

    trees
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_one(include_str!("example.txt")), 21);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 1733);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 284648);
    }
}
