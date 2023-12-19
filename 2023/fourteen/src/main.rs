use std::collections::HashSet;

fn main() {
    let rocks_map = include_str!("rocks.input").split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    
    let north_rocks = move_all_rocks_north(rocks_map.clone());
    let rocks_load: usize =  get_north_load(&north_rocks);
    println!("rocks load: {}", rocks_load);

    let cycled_map = nth_cycle(rocks_map.clone(), 1000000000);
    let rocks_load_cycled = get_north_load(&cycled_map);
    println!("rocks load cycled: {}", rocks_load_cycled);
}

fn get_north_load(map: &[Vec<u8>]) -> usize {
    get_rock_indecies(&map).iter()
        .map(|(row, _)|  map[0].len() - row)
        .sum()
}

fn nth_cycle(mut map: Vec<Vec<u8>>,n: usize) -> Vec<Vec<u8>> {
    let (cycle_start, cycle_length) = cycle_start_and_length(map.clone());
    for _ in 1..=cycle_start {
        map = cycle_rocks(map);
    }
    let cycles_to_do = (n - cycle_start) % cycle_length;
    for _ in 1..=cycles_to_do {
        map = cycle_rocks(map);
    }
    map
}

fn cycle_start_and_length(map: Vec<Vec<u8>>) -> (usize,usize) { 
    let mut visited = HashSet::new();
    let mut cycled_rocks = cycle_rocks(map.clone());
    let mut cycle_length = 0;
    let mut start_of_cylce = 0;
    visited.insert(cycled_rocks.clone());
    for i in 1.. {
        cycled_rocks = cycle_rocks(cycled_rocks);
        if visited.contains(&cycled_rocks) && start_of_cylce > 0{
            cycle_length = i + 1 - start_of_cylce;
            break;
        }
        if visited.contains(&cycled_rocks) {
            visited.clear();
            start_of_cylce = i + 1;
        }
        visited.insert(cycled_rocks.clone());
    }
    (start_of_cylce - cycle_length, cycle_length)
}

fn cycle_rocks(mut map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    map = move_all_rocks_north(map);
    map = move_all_rocks_west(map);
    map = move_all_rocks_south(map);
    move_all_rocks_east(map)
}

fn get_rock_indecies(map: &[Vec<u8>]) -> Vec<(usize,usize)>{
    let mut rocks_indecies = vec![];
    for (row_index, row) in map.iter().enumerate() {
        for (column_index, &object) in row.iter().enumerate() {
            if object == b'O' {
                rocks_indecies.push((row_index,column_index));
            }
        }
    }
    rocks_indecies
}

fn draw_rocks_map(map: &[Vec<u8>]) {
    for row in map.iter() {
        for object in row.iter() {
            print!("{}", *object as char)
        }
        println!()
    }
}

fn move_all_rocks_north(mut map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for row_index in 0..map.len() {
        for column_index in 0..map[row_index].len(){
            if map[row_index][column_index] == b'O' {
                map = move_rock_north(map, (row_index,column_index));
            }
        }
    }
    map
}

fn move_rock_north(mut new_map: Vec<Vec<u8>>, location: (usize, usize)) 
    -> Vec<Vec<u8>> {
    for delta in 1..=location.0 {
        if new_map[location.0 - delta][location.1] != b'.' {
            new_map[location.0][location.1] = b'.';
            new_map[location.0 - delta + 1][location.1] = b'O';
            return new_map;
        }
    }
    new_map[location.0][location.1] = b'.';
    new_map[0][location.1] = b'O';
    new_map
}

fn move_all_rocks_south(mut map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for row_index in (0..map.len()).rev() {
        for column_index in 0..map[row_index].len(){
            if map[row_index][column_index] == b'O' {
                map = move_rock_south(map, (row_index,column_index));
            }
        }
    }
    map
}

fn move_rock_south(mut new_map: Vec<Vec<u8>>, location: (usize, usize)) 
    -> Vec<Vec<u8>> {
    let south_max_index = new_map.len() - 1;
    for delta in 1..=south_max_index - location.0 {
        if new_map[location.0 + delta][location.1] != b'.' {
            new_map[location.0][location.1] = b'.';
            new_map[location.0 + delta - 1][location.1] = b'O';
            return new_map;
        }
    }
    new_map[location.0][location.1] = b'.';
    new_map[south_max_index][location.1] = b'O';
    new_map
}

fn move_all_rocks_east(mut map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for column_index in (0..map[0].len()).rev() {
        for row_index in 0..map.len() {
            if map[row_index][column_index] == b'O' {
                map = move_rock_east(map, (row_index,column_index));
            }
        }
    }
    map
}

fn move_rock_east(mut new_map: Vec<Vec<u8>>, location: (usize, usize)) 
    -> Vec<Vec<u8>> {
    let east_max_index = new_map[0].len() - 1;
    for delta in 1..=east_max_index - location.1 {
        if new_map[location.0][location.1 + delta] != b'.' {
            new_map[location.0][location.1] = b'.';
            new_map[location.0][location.1 + delta - 1] = b'O';
            return new_map;
        }
    }
    new_map[location.0][location.1] = b'.';
    new_map[location.0][east_max_index] = b'O';
    new_map
}

fn move_all_rocks_west(mut map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for column_index in 0..map[0].len() {
        for row_index in 0..map.len() {
            if map[row_index][column_index] == b'O' {
                map = move_rock_west(map, (row_index,column_index));
            }
        }
    }
    map
}

fn move_rock_west(mut new_map: Vec<Vec<u8>>, location: (usize, usize)) 
    -> Vec<Vec<u8>> {
    for delta in 1..=location.1 {
        if new_map[location.0][location.1 - delta] != b'.' {
            new_map[location.0][location.1] = b'.';
            new_map[location.0][location.1 - delta + 1] = b'O';
            return new_map;
        }
    }
    new_map[location.0][location.1] = b'.';
    new_map[location.0][0] = b'O';
    new_map
}
