


fn main() {
    let galaxy_map = parse_galaxy(include_str!("galaxy-map.input"));
    
    println!("free: space {:?}", get_free_space(&galaxy_map));

}

fn expand_universe(mut galaxy_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    
    

    for column in get_free_columns(&galaxy_map) {
        galaxy_map = double_column(column, galaxy_map);
    }

    for row in get_free_rows(&galaxy_map){
        galaxy_map = double_row(row, galaxy_map);
    }

    galaxy_map
}

fn double_row(row: usize, mut galaxy_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    galaxy_map.insert(row, galaxy_map[row].clone());
    galaxy_map
}

fn double_column(column: usize, mut galaxy_map: Vec<Vec<char>>) 
    -> Vec<Vec<char>> {
    for row in galaxy_map.iter_mut() {
        row.insert(column, row[column]);
    }
    galaxy_map
}

fn parse_galaxy(raw: &str) -> Vec<Vec<char>> {
     raw.split("\n")
        .map(|x| Vec::from_iter(x.chars()))
        .collect::<Vec<Vec<char>>>()
}

fn get_free_space(galaxy_map: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut free_rows = vec![];
    
    for row in 0..galaxy_map.len() {
        if galaxy_map[row].iter().all(|&x| x == '.') {
            free_rows.push(row);
        }
    }
    
    let mut free_columns = vec![];

    for column in 0..galaxy_map[0].len() {
        if galaxy_map.iter().map(|x| x[column]).all(|x| x == '.') {
            free_columns.push(column);
        }
    }

    (free_rows, free_columns)
}

fn get_free_rows(galaxy_map: &Vec<Vec<char>>) -> Vec<usize> {
    galaxy_map.iter().enumerate()
        .filter(|(_,x)| x.iter().all(|&x| x == '.'))
        .map(|(index,_)| index)
        .collect()
}

fn get_free_columns(galaxy_map: &Vec<Vec<char>>) -> Vec<usize> {
    let mut free_columns = vec![];
    for column in 0..galaxy_map[0].len() {
        if galaxy_map.iter().map(|x| x[column]).all(|x| x == '.') {
            free_columns.push(column);
        }
    }
    free_columns
}


#[cfg(test)]
mod tests {
    use super::*;
    const PART_ONE: &str = 
        "...#......\n\
         .......#..\n\
         #.........\n\
         ..........\n\
         ......#...\n\
         .#........\n\
         .........#\n\
         ..........\n\
         .......#..\n\
         #...#.....";

    const PART_ONE_EXPANDED: &str =
       "....#........\n\
        .........#...\n\
        #............\n\
        .............\n\
        .............\n\
        ........#....\n\
        .#...........\n\
        ............#\n\
        .............\n\
        .............\n\
        .........#...\n\
        #....#.......";

    #[test]
    fn part1() {
        let galaxy = parse_galaxy(PART_ONE);
        let expanded_galaxy = parse_galaxy(PART_ONE_EXPANDED);
        let (free_rows, free_columns) = (
                get_free_rows(&galaxy),
                get_free_columns(&galaxy)
            );
        assert_eq!((&free_rows, &free_columns), (&vec![3,7], &vec![2,5,8]));
        assert_eq!(
            expand_universe(galaxy),
            expanded_galaxy
        );
    }
}
