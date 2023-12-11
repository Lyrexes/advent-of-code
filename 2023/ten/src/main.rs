use std::collections::HashSet;

#[derive(Default,Debug, Clone)]
struct Pipe {
    pipe_kind: u8,
    neighbours: Vec<Option<usize>>
}

#[derive(Debug)]
struct Rect {
    top_left: (usize,usize),
    bottom_right: (usize,usize)
}

fn main() {
    let pipes_raw = include_str!("pipes.input").split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.into())
        .collect::<Vec<Vec<u8>>>();
    let pipes = parse_pipes(&pipes_raw);
    let giant_loop = get_giant_loop_length(&pipes);
    println!("giatn loop: {:?}", giant_loop.len() / 2);
    println!(
        "rect around loop: {:?}", 
         get_rect_around_loop(&giant_loop, &pipes_raw)
    );
    
}

fn get_fully_enclosed_by(giant_loop: &Vec<usize>, pipes_raw: &Vec<Vec<u8>>) 
    -> Vec<usize> {
    let rect = get_rect_around_loop(&giant_loop, &pipes_raw);
    let loop_indices: HashSet<usize> = HashSet::from_iter(
        giant_loop.iter().cloned()
    );
    let mut visited: HashSet<usize> = HashSet::new();
    let mut enclosed = vec![];
    
    for row in rect.top_left.0..rect.bottom_right.0 {
        for column in rect.top_left.1..rect.bottom_right.1 {
            if visited.contains(&(row * pipes_raw.len() + column)) {
                continue;
            }
            search_for_exit(
                row, 
                column,
                &mut visited,
                &mut enclosed, 
                &loop_indices,
                &rect
            );
        }
    }
    enclosed
}

fn search_for_exit(row: usize, column: usize, visited: &mut HashSet<usize>,
                   enclosed: &mut Vec<usize>, loop_indices: &HashSet<usize>,
                   border: &Rect) {

    let mut current_visited = HashSet::new();
    let mut to_be_visited = HashSet::new();
    to_be_visited.insert((row,column));
    let mut current_node;
    loop {
        if to_be_visited.is_empty() {
            break;
        }
            
        current_node = to_be_visited.take(
            &to_be_visited.iter().nth(0).unwrap().clone()
        ).unwrap();

        if is_adjacent_to_border(&current_node, border) {
            enclosed.clear();
            return;
        }

        to_be_visited.extend(
            get_neighbours_node(
                &current_node, 
                &current_visited, 
                &loop_indices
            ).iter()
        )
    }
}

fn get_neighbours_node(current_node: &(usize, usize), 
                       current_visited: &HashSet<usize>, 
                       loop_indices: &HashSet<usize>) -> Vec<(usize,usize)> {
    let mut neighbors = vec![];
    let mut current_neighbor;
    for row_add in [-1,1] {
        for column_add in [-1,1] {
            
        }
    }
    todo!()
}

fn is_adjacent_to_border(current_node: &(usize, usize), border: &Rect) -> bool {
    todo!()
}


fn get_rect_around_loop(giant_loop: &Vec<usize>, pipes_raw: &Vec<Vec<u8>>)
    -> Rect {
    let left = giant_loop.iter()
        .map(|&x| x % pipes_raw.len())
        .min()
        .unwrap();
    let right = giant_loop.iter()
        .map(|&x| x % pipes_raw.len())
        .max()
        .unwrap();
    let top = giant_loop.iter()
        .map(|x| x / pipes_raw.len())
        .inspect(|x| println!("{}", x))
        .min()
        .unwrap();
    let bottom = giant_loop.iter()
        .map(|x| x / pipes_raw.len())
        .max()
        .unwrap();
    Rect {
        top_left: (top, left),
        bottom_right: (bottom, right)
    }
}   

fn get_giant_loop_length(pipes: &Vec<Pipe>) -> Vec<usize> {
    let start_index = pipes.iter().position(|x| x.pipe_kind == b'S').unwrap();
    let mut cycles = vec![];
    for &start_neighbour in pipes[start_index].neighbours.iter().flatten() {
        let mut current_pipe = start_neighbour;
        let mut previous_pipe = start_index;
        let mut current_cycle = vec![start_index];
        while pipes[current_pipe].pipe_kind != b'S' {

            let maybe_next_pipe = pipes[current_pipe].neighbours.iter()
                .flatten()
                .position(|&x| x != previous_pipe)
                .map(|x| pipes[current_pipe].neighbours[x].unwrap());

            if let Some(pipe) = maybe_next_pipe {
                previous_pipe = current_pipe;
                current_pipe = pipe;
            } else {
                current_cycle.clear();
                break
            }
            current_cycle.push(current_pipe)
        }
        cycles.push(current_cycle);
    }
    cycles.into_iter().max_by_key(|x| x.len()).unwrap()
}

fn parse_pipes(pipes_raw: &Vec<Vec<u8>>) -> Vec<Pipe> {

    let mut pipes = vec![Pipe::default();pipes_raw.len() * pipes_raw[0].len()];

    for row in 0..pipes_raw.len() {
        for column in 0..pipes_raw[0].len() {
            let i = row * pipes_raw[0].len() + column;
            match pipes_raw [row][column] {
                b'|' => pipes[i] = vertical_pipe(&pipes_raw, row, column),
                b'-' => pipes[i] = horizontal_pipe(&pipes_raw, row, column),
                b'L' => pipes[i] = north_east_bend(&pipes_raw, row, column),
                b'J' => pipes[i] = north_west_bend(&pipes_raw, row, column),
                b'7' => pipes[i] = south_west_bend(&pipes_raw, row, column),
                b'F' => pipes[i] = south_east_bend(&pipes_raw, row, column),
                b'S' => pipes[i] = start_pipe(&pipes_raw, row, column),
                _ => {}
            }
        }
    }
    pipes
}

fn is_in_bounds(matrix: &Vec<Vec<u8>>, row: i64, column: i64) -> bool {
    (0..matrix.len() as i64).contains(&row) 
        && (0..matrix[0].len() as i64).contains(&column)
}

fn vertical_pipe(matrix: &Vec<Vec<u8>>, row: usize, column: usize) -> Pipe {
    Pipe {
        pipe_kind: b'|',
        neighbours: vec![
            get_neighbour(matrix, row as i64 + 1, column as i64),
            get_neighbour(matrix, row as i64 - 1, column as i64)
        ]
    }
}

fn horizontal_pipe(matrix: &Vec<Vec<u8>>, row: usize, column: usize) -> Pipe {
    Pipe {
        pipe_kind: b'-',
        neighbours: vec![
            get_neighbour(matrix, row as i64, column as i64 + 1),
            get_neighbour(matrix, row as i64, column as i64 - 1)
        ]
    }
}

fn north_east_bend(matrix: &Vec<Vec<u8>>, row: usize, column: usize) -> Pipe {
    Pipe {
        pipe_kind: b'L',
        neighbours: vec![
            get_neighbour(matrix, row as i64, column as i64 + 1),
            get_neighbour(matrix, row as i64 - 1, column as i64)
        ]
    }
}

fn north_west_bend(matrix: &Vec<Vec<u8>>, row: usize, column: usize) -> Pipe {
    Pipe {
        pipe_kind: b'J',
        neighbours: vec![
            get_neighbour(matrix, row as i64, column as i64 - 1),
            get_neighbour(matrix, row as i64 - 1, column as i64)
        ]
    }
}

fn south_west_bend(matrix: &Vec<Vec<u8>>, row: usize, column: usize) -> Pipe {
    Pipe {
        pipe_kind: b'7',
        neighbours: vec![
            get_neighbour(matrix, row as i64, column as i64 - 1),
            get_neighbour(matrix, row as i64 + 1, column as i64)
        ]
    }
}

fn south_east_bend(matrix: &Vec<Vec<u8>>, row: usize, column: usize) -> Pipe {
    Pipe {
        pipe_kind: b'F',
        neighbours: vec![
            get_neighbour(matrix, row as i64, column as i64 + 1),
            get_neighbour(matrix, row as i64 + 1, column as i64)
        ]
    }
}

fn start_pipe(matrix: &Vec<Vec<u8>>, row: usize, column: usize) -> Pipe {
    Pipe {
        pipe_kind: b'S',
        neighbours: vec![
            get_neighbour(matrix, row as i64, column as i64 + 1),
            get_neighbour(matrix, row as i64, column as i64 - 1),
            get_neighbour(matrix, row as i64 + 1, column as i64),
            get_neighbour(matrix, row as i64 - 1, column as i64)
        ]
    }
}

fn get_neighbour(matrix: &Vec<Vec<u8>>, row: i64, column: i64) 
    -> Option<usize> {
    if is_in_bounds(matrix, row, column) 
            && matrix[row as usize][column as usize] != b'.'{
        Some((row as usize) * matrix[0].len() + column as usize)
    } else {
        None
    }
}

fn get_neighbour_groud(matrix: &Vec<Vec<u8>>, row: i64, column: i64) 
    -> Option<usize> {
    if is_in_bounds(matrix, row, column) {
        Some((row as usize) * matrix[0].len() + column as usize)
    } else {
        None
    }
}
