use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let tree_map = parse_tree_map("map")?;
    println!("trees visible: {}", get_visible_trees(&tree_map));
    println!(
        "get highest scenic score: {}",
        get_highest_scenic_score(&tree_map)
    );

    Ok(())
}

fn parse_tree_map(file: &str) -> io::Result<Box<[Box<[u8]>]>> {
    Ok(BufReader::new(File::open(file)?)
        .lines()
        .into_iter()
        .flatten()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10u32))
                .flatten()
                .map(|x| x as u8)
                .collect::<Vec<u8>>()
                .into_boxed_slice()
        })
        .collect::<Vec<Box<[u8]>>>()
        .into_boxed_slice())
}

fn get_highest_scenic_score(tree_map: &Box<[Box<[u8]>]>) -> u32 {
    let mut max_scenic_score = u32::MIN;
    let mut current_scenic_score;
    for row in 0..tree_map.len() {
        for column in 0..tree_map[0].len() {
            current_scenic_score = get_scenic_score(row, column, tree_map);
            if current_scenic_score > max_scenic_score {
                max_scenic_score = current_scenic_score;
            }
        }
    }
    max_scenic_score
}

fn get_visible_trees(tree_map: &Box<[Box<[u8]>]>) -> u32 {
    let mut visible_count = 0;
    for row in 0..tree_map.len() {
        for column in 0..tree_map[0].len() {
            if column == 0
                || row == 0
                || column == tree_map[0].len() - 1
                || row == tree_map.len() - 1
            {
                visible_count += 1;
                continue;
            }
            visible_count += (is_higher_all_right(row, column, &tree_map)
                || is_higher_all_left(row, column, &tree_map)
                || is_higher_all_up(row, column, &tree_map)
                || is_higher_all_down(row, column, &tree_map)) as u32;
        }
    }
    visible_count
}

fn get_scenic_score(row: usize, column: usize, map: &Box<[Box<[u8]>]>) -> u32 {
    if column == 0 || row == 0 || column == map[0].len() - 1 || row == map.len() - 1 {
        return 0;
    }
    get_scenic_score_up(row, column, map)
        * get_scenic_score_down(row, column, map)
        * get_scenic_score_left(row, column, map)
        * get_scenic_score_right(row, column, map)
}

fn get_scenic_score_right(row: usize, column: usize, map: &Box<[Box<[u8]>]>) -> u32 {
    let count = map[row][column + 1..]
        .iter()
        .take_while(|x| **x < map[row][column])
        .count() as u32;
    if count == map[row][column + 1..].len() as u32 {
        count
    } else {
        count + 1
    }
}

fn get_scenic_score_left(row: usize, column: usize, map: &Box<[Box<[u8]>]>) -> u32 {
    let count = map[row][..column]
        .iter()
        .rev()
        .take_while(|x| **x < map[row][column])
        .count() as u32;

    if count == map[row][..column].len() as u32 {
        count
    } else {
        count + 1
    }
}

fn get_scenic_score_up(row: usize, column: usize, map: &Box<[Box<[u8]>]>) -> u32 {
    let count = map[..row]
        .iter()
        .rev()
        .take_while(|x| (**x)[column] < map[row][column])
        .count() as u32;

    if count == map[..row].len() as u32 {
        count
    } else {
        count + 1
    }
}

fn get_scenic_score_down(row: usize, column: usize, map: &Box<[Box<[u8]>]>) -> u32 {
    let count = map[row + 1..]
        .iter()
        .take_while(|x| (**x)[column] < map[row][column])
        .count() as u32;

    if count == map[row+1..].len() as u32 {
        count
    } else {
        count + 1
    }
}

fn is_higher_all_right(row: usize, column: usize, map: &Box<[Box<[u8]>]>) -> bool {
    map[row][column + 1..].iter().all(|x| *x < map[row][column])
}

fn is_higher_all_left(row: usize, column: usize, map: &Box<[Box<[u8]>]>) -> bool {
    map[row][..column].iter().all(|x| *x < map[row][column])
}

fn is_higher_all_up(row: usize, column: usize, map: &Box<[Box<[u8]>]>) -> bool {
    map[..row].iter().all(|x| x[column] < map[row][column])
}

fn is_higher_all_down(row: usize, column: usize, map: &Box<[Box<[u8]>]>) -> bool {
    map[row + 1..].iter().all(|x| x[column] < map[row][column])
}
