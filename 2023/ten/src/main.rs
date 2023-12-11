use std::collections::HashSet;

#[derive(Default,Debug, Clone)]
struct Pipe {
    pipe_kind: u8,
    neighbours: Vec<Option<usize>>
}

fn main() {
    let pipes = parse_pipes(include_str!("pipes.input"));
    println!("giatn loop: {:?}", get_giant_loop_length(&pipes) / 2);
}

fn get_giant_loop_length(pipes: &Vec<Pipe>) -> usize {
    let start_index = pipes.iter().position(|x| x.pipe_kind == b'S').unwrap();
    let mut cycles_length = vec![];
    for &start_neighbour in pipes[start_index].neighbours.iter().flatten() {
        let mut current_pipe = start_neighbour;
        let mut previous_pipe = start_index;
        let mut cycle_counter = 1;
        while pipes[current_pipe].pipe_kind != b'S' {

            let maybe_next_pipe = pipes[current_pipe].neighbours.iter()
                .flatten()
                .position(|&x| x != previous_pipe)
                .map(|x| pipes[current_pipe].neighbours[x].unwrap());

            if let Some(pipe) = maybe_next_pipe{
                previous_pipe = current_pipe;
                current_pipe = pipe;
            } else {
                cycle_counter = 0;
                break
            }
            cycle_counter += 1;
        }
        cycles_length.push(cycle_counter);
    }
    *cycles_length.iter().max().unwrap() as usize
}

fn parse_pipes(raw: &str) -> Vec<Pipe> {
    let pipes_raw = raw.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.into())
        .collect::<Vec<Vec<u8>>>();

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
                _ => pipes[i] = ground(&pipes_raw, row, column)
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

fn ground(matrix: &Vec<Vec<u8>>, row: usize, column: usize) -> Pipe {
    Pipe {
        pipe_kind: b'.',
        neighbours: vec![
            get_neighbour(matrix, row as i64, column as i64 + 1),
            get_neighbour(matrix, row as i64, column as i64 - 1),
            get_neighbour(matrix, row as i64 + 1, column as i64),
            get_neighbour(matrix, row as i64 - 1, column as i64),
            get_neighbour(matrix, row as i64 + 1, column as i64 + 1),
            get_neighbour(matrix, row as i64 - 1, column as i64 - 1),
            get_neighbour(matrix, row as i64 + 1, column as i64 - 1),
            get_neighbour(matrix, row as i64 - 1, column as i64 + 1)
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
