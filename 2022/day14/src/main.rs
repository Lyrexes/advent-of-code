use std::{thread, time};

#[derive(Debug, Clone, Copy)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Wall {
    Stone,
    Air,
    Sand,
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} -> {},{}",
            self.start.0, self.start.1, self.end.0, self.end.1
        )
    }
}

fn main() {
    void_simulation();
    floor_simulation();
}

fn floor_simulation() {
    let mut lines = include_str!("scan")
        .lines()
        .map(parse_lines)
        .flatten()
        .collect::<Vec<Line>>();
    
    lines = add_floor_line(lines);
    let starting_point_x = normalize_x(&mut lines);
    let (width, height) = get_dimemsions(&lines);

    let mut map = (vec![(vec![Wall::Air; width]).into_boxed_slice(); height]).into_boxed_slice();
    map = generate_rocks(map, lines);

    let mut is_falling = false;
    let mut current_sand = (0, starting_point_x);
    let mut sand_count = 0;
    //let pause_time = time::Duration::from_secs(1);
    loop {
        if !is_falling {
            is_falling = true;
            map[0][starting_point_x] = Wall::Sand;
            current_sand = (0, starting_point_x);
        } else if let Some(new_position) = move_sand(current_sand, &map) {
            if new_position == current_sand {
                sand_count += 1;
                if current_sand == (0,starting_point_x) {
                    break;
                }
                is_falling = false;
            } else {
                map[current_sand.0][current_sand.1] = Wall::Air;
                map[new_position.0][new_position.1] = Wall::Sand;
                current_sand = new_position;
            }
        }
        //print_map(&map);
        //thread::sleep(pause_time);
    }
    print_map(&map);
    println!("sand_count: {}", sand_count);
}

fn add_floor_line(mut lines: Vec<Line>) -> Vec<Line> {
    let min_x = usize::min(
        lines.iter().map(|x| x.start.0).min().unwrap(),
        lines.iter().map(|x| x.end.0).min().unwrap(),
    );
    let max_x = usize::max(
        lines.iter().map(|x| x.start.0).max().unwrap(),
        lines.iter().map(|x| x.end.0).max().unwrap(),
    );
    let max_y = usize::max(
        lines.iter().map(|x| x.start.1).max().unwrap(),
        lines.iter().map(|x| x.end.1).max().unwrap(),
    );
    lines.push(Line { start: (min_x- 3*(min_x.abs_diff(max_x)),max_y+2), 
        end: (min_x + 3*(min_x.abs_diff(max_x)), max_y+2) });
    lines
}

fn void_simulation() {
    let mut lines = include_str!("scan")
        .lines()
        .map(parse_lines)
        .flatten()
        .collect::<Vec<Line>>();
    
    let starting_point_x = normalize_x(&mut lines);
    let (width, height) = get_dimemsions(&lines);

    let mut map = (vec![(vec![Wall::Air; width]).into_boxed_slice(); height]).into_boxed_slice();
    map = generate_rocks(map, lines);

    let mut is_falling = false;
    let mut current_sand = (0, starting_point_x);
    let mut sand_count = 0;
    //let pause_time = time::Duration::from_secs(1);
    loop {
        if !is_falling {
            is_falling = true;
            map[0][starting_point_x] = Wall::Sand;
            current_sand = (0, starting_point_x);
        } else if let Some(new_position) = move_sand(current_sand, &map) {
            if new_position == current_sand {
                is_falling = false;
                sand_count += 1;
            } else {
                map[current_sand.0][current_sand.1] = Wall::Air;
                map[new_position.0][new_position.1] = Wall::Sand;
                current_sand = new_position;
            }
        } else {
            break;
        }
        //print_map(&map);
        //thread::sleep(pause_time);
    }
    print_map(&map);
    println!("sand_count: {}", sand_count);
}

fn move_sand(old_position: (usize, usize), map: &Box<[Box<[Wall]>]>) -> Option<(usize, usize)> {
    let new_position_down = (old_position.0 + 1, old_position.1);
    let new_position_down_left = (old_position.0 + 1, old_position.1 - 1);
    let new_position_down_right = (old_position.0 + 1, old_position.1 + 1);

    for new_position in [
        new_position_down,
        new_position_down_left,
        new_position_down_right,
    ]
    .iter()
    {
        if is_out_of_bounds(new_position, map) {
            return None;
        }
        if map[new_position.0][new_position.1] == Wall::Air {
            return Some(*new_position);
        }
    }
    Some(old_position)
}

fn is_out_of_bounds(position: &(usize, usize), map: &Box<[Box<[Wall]>]>) -> bool {
    !(0..map.len()).contains(&position.0) || !(0..map[0].len()).contains(&position.1)
}

fn generate_rocks(mut map: Box<[Box<[Wall]>]>, lines: Vec<Line>) -> Box<[Box<[Wall]>]> {
    for line in lines.iter() {
        let start_row = usize::min(line.start.1, line.end.1);
        let start_column = usize::min(line.start.0, line.end.0);
        let delta_column = line.start.0.abs_diff(line.end.0);
        let delta_row = line.start.1.abs_diff(line.end.1);
        for column in 0..=delta_column {
            for row in 0..=delta_row {
                map[start_row + row][start_column + column] = Wall::Stone;
            }
        }
    }
    map
}

fn print_map(map: &Box<[Box<[Wall]>]>) {
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            match map[row][column] {
                Wall::Stone => print!("#"),
                Wall::Air => print!("."),
                Wall::Sand => print!("o"),
            }
        }
        println!()
    }
}

fn get_dimemsions(lines: &Vec<Line>) -> (usize, usize) {
    let max_x = usize::max(
        lines.iter().map(|x| x.start.0).max().unwrap(),
        lines.iter().map(|x| x.end.0).max().unwrap(),
    );
    let max_y = usize::max(
        lines.iter().map(|x| x.start.1).max().unwrap(),
        lines.iter().map(|x| x.end.1).max().unwrap(),
    );
    (max_x as usize + 1, max_y as usize + 1)
}

fn normalize_x(lines: &mut Vec<Line>) -> usize {
    let min_x = usize::min(
        lines.iter().map(|x| x.start.0).min().unwrap(),
        lines.iter().map(|x| x.end.0).min().unwrap(),
    );

    *lines = lines
        .into_iter()
        .map(|x| Line {
            start: (x.start.0.abs_diff(min_x), x.start.1),
            end: (x.end.0.abs_diff(min_x), x.end.1),
        })
        .collect::<Vec<Line>>();
    min_x.abs_diff(500)
}

fn parse_lines(raw: &str) -> Vec<Line> {
    let points = raw.split(" -> ").map(parse_point);
    points
        .clone()
        .zip(points.skip(1))
        .map(|(start, end)| Line { start, end })
        .collect::<Vec<Line>>()
}

fn parse_point(raw: &str) -> (usize, usize) {
    let (x, y) = raw.split_at(raw.find(',').unwrap());
    (x.parse().unwrap(), y.trim_matches(',').parse().unwrap())
}
