fn main() {
    let rocks_map = include_str!("rocks.input").split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let north_rocks = move_all_rocks_north(&rocks_map);
    
    //draw_rocks_map(&rocks_map);
    draw_rocks_map(&north_rocks);

    let rocks_load: usize = get_rock_indecies(&north_rocks).iter()
        .map(|(row, _)|  north_rocks[0].len() - row)
        .sum();
    println!("rocks load: {}", rocks_load);
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

fn move_all_rocks_north(map: &[Vec<u8>])  -> Vec<Vec<u8>> {
    let mut new_map = vec![];
    map.clone_into(&mut new_map);
    for (row_index,row) in map.iter().enumerate() {
        for (column_index, &object) in row.iter().enumerate() {
            if object == b'O' {
                new_map = move_rock_north(new_map, (row_index,column_index));
            }
        }
    }
    new_map
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
