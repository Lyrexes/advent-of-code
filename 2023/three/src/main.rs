fn main() {
     let engine_map = include_str!("engine-map")
        .lines()
        .collect::<Vec<&str>>();
    let parts_sum: u64 = get_part_number_indices(&engine_map).iter()
        .map(|&i| number_index_to_number(&engine_map, i))
        .sum();
    println!("parts: {}", parts_sum);

    let gears_sum: u64 = get_gear_ratio_indices(&engine_map).iter()
        .map(|&i| (
                number_index_to_number(&engine_map,i.0),
                number_index_to_number(&engine_map,i.1)
            )
        )
        .map(|(x,y)| x*y)
        .sum();
    println!("gears: {}", gears_sum);
}

fn get_gear_ratio_indices(engine_map: &Vec<&str>) 
        -> Vec<((usize,usize),(usize,usize))> {
    let mut gear_ratios = vec![];
    for (index_y, row) in engine_map.iter().enumerate() {
        for (index_x, element) in row.bytes().enumerate() {
            if element != b'*' {
                continue;
            }
            
            if let Some(ratio) = get_ratio(index_x, index_y, &engine_map) {
                gear_ratios.push(ratio)
            }
        }
    }
    gear_ratios   
}

fn get_ratio(x: usize, y: usize, engine_map: &Vec<&str>) 
    -> Option<((usize,usize),(usize,usize))> {
    let mut adjacent_numbers = vec![];
    let mut current_x;
    let mut current_y;

    for add_x in [-1,0,1] {
        for add_y in [-1,0,1] {
            current_x = x as i32 + add_x;
            current_y = y as i32 + add_y;

            
            if !(0..engine_map[0].len() as i32).contains(&current_x)
                || !(0..engine_map.len() as i32).contains(&current_y) {
                    continue;
            }
            
            let neighbor = engine_map[current_y as usize].chars()
                    .nth(current_x as usize)
                    .unwrap();

            if neighbor.is_ascii_digit() {
                adjacent_numbers.push(
                    get_starting_index_of_number(
                        current_x as usize,
                        current_y as usize,
                        &engine_map
                    )
                );
            }
        }
    }
    adjacent_numbers.sort();
    adjacent_numbers.dedup();
    
    if adjacent_numbers.len() == 2 {
        Some((adjacent_numbers[0], adjacent_numbers[1]))
    } else {
        None
    }
}

fn get_starting_index_of_number(x: usize, y: usize, engine_map: &Vec<&str>)
    -> (usize,usize) {
    let mut index_beginning = x as i32;
    loop {
        if index_beginning < 0 {
            index_beginning += 1;
            break;
        } 
        if engine_map[y].bytes().nth(index_beginning as usize).unwrap()
            .is_ascii_digit() {
                index_beginning -= 1;
        } else {
            index_beginning += 1;
            break;
        }
    }
    (index_beginning as usize, y)
}


fn number_index_to_number(engine_map: &Vec<&str>, index: (usize,usize)) -> u64 {
    let mut index_beginning = index.0 as i32;
    loop {
        if index_beginning < 0 {
            index_beginning += 1;
            break;
        } 
        if engine_map[index.1].bytes().nth(index_beginning as usize).unwrap()
            .is_ascii_digit() {
                index_beginning -= 1;
        } else {
            index_beginning += 1;
            break;
        }
    }

    engine_map[index.1].get((index_beginning as usize)..).unwrap()
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn get_part_number_indices(engine_map: &Vec<&str>) -> Vec<(usize,usize)> {
    let mut adjacent_number_indices = vec![];
    let mut number_has_neighbor = false;
    for (index_y, row) in engine_map.iter().enumerate() {
        for (index_x, element) in row.bytes().enumerate() {
            if !element.is_ascii_digit() {
                number_has_neighbor = false;
                continue;
            }
            if number_has_neighbor {
                continue;
            }
            
            if has_symbol_neighbor(index_x, index_y, &engine_map) {
                adjacent_number_indices.push((index_x, index_y));
                number_has_neighbor = true;
            }
        }
        number_has_neighbor = false;
    }
    adjacent_number_indices
}

fn has_symbol_neighbor(x: usize, y: usize, engine_map: &Vec<&str>) -> bool {
    let no_symbol = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
    let mut current_x;
    let mut current_y;

    for add_x in [-1,0,1] {
        for add_y in [-1,0,1] {
            current_x = x as i32 + add_x;
            current_y = y as i32 + add_y;

            
            if !(0..engine_map[0].len() as i32).contains(&current_x)
                || !(0..engine_map.len() as i32).contains(&current_y) {
                    continue;
            }
            
            let neighbor = engine_map[current_y as usize].chars()
                    .nth(current_x as usize)
                    .unwrap();

            if !no_symbol.contains(&neighbor) {
                return true
            }

        }
    }
    false
}
