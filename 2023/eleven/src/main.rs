
#[derive(Debug,Clone,Copy)]
enum UniverseNode {
    EmptySpace,
    Galaxy(u16)
}

fn main() {
    let mut galaxy_map = parse_galaxy(include_str!("galaxy-map.input"));
    galaxy_map = expand_universe(galaxy_map);
    let universe = enumerate_universe(&galaxy_map);
    draw_universe(&universe);
}

fn expand_universe(mut galaxy_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for (added,column) in get_free_columns(&galaxy_map).iter().enumerate() {
        galaxy_map = double_column(column + added, galaxy_map);
    }
    for (added,row) in get_free_rows(&galaxy_map).iter().enumerate() {
        galaxy_map = double_row(row + added, galaxy_map);
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
        .filter(|x| !x.is_empty())
        .map(|x| Vec::from_iter(x.chars()))
        .collect::<Vec<Vec<char>>>()
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

fn get_unique_pairs(amount: usize) -> Vec<(usize,usize)> {
    let cartesian_product = (1..amount)
        .map(|row| (1..amount).map(move |column| (row,column)))
        .flatten()
        .collect::<Vec<(usize,usize)>>();
    cartesian_product.iter()
        .filter(|(x,y)| !cartesian_product.contains(&(*y,*x)))
        .map(|(x,y)| (*x,*y))
        .collect()
}

fn draw_galaxy(galaxy_map: &Vec<Vec<char>>) {
    for row in galaxy_map.iter() {
        println!("{}", String::from_iter(row.iter()))
    }
}

fn draw_universe(galaxy_map: &Vec<Vec<UniverseNode>>) {
    for row in galaxy_map.iter() {
        for &column in row.iter() {
            match column {
                UniverseNode::EmptySpace => print!("."),
                UniverseNode::Galaxy(num) => print!("{}", num.to_string()),
            }
        }
        println!()
    }
}

fn enumerate_universe(galaxy_map: &Vec<Vec<char>>) -> Vec<Vec<UniverseNode>> {
    let mut enumerated_galaxy = vec![];
    let mut enumerated_galaxy_row;
    let mut counter = 1;
    for row in galaxy_map.iter() {
        enumerated_galaxy_row = vec![];
        for &column in row.iter() {
            if column == '#' {
                enumerated_galaxy_row.push(UniverseNode::Galaxy(counter));
                counter += 1;
            } else {
                enumerated_galaxy_row.push(UniverseNode::EmptySpace);
            }
        }
        enumerated_galaxy.push(enumerated_galaxy_row);
    }
    enumerated_galaxy
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
        assert_eq!(
            (get_free_rows(&galaxy), get_free_columns(&galaxy)),
            (vec![3,7], vec![2,5,8])
        );

        assert_eq!(
            expand_universe(galaxy.clone()),
            expanded_galaxy
        );

        assert_eq!(get_unique_pairs(9).len(), 36);
    }
}
