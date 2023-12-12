use std::{collections::{HashSet, HashMap}, ops::Range};


#[derive(Debug,Clone,Copy)]
enum UniverseNode {
    EmptySpace,
    Galaxy(u16)
}

fn main() {
    let mut galaxy_map = parse_galaxy(include_str!("galaxy-map.input"));
    galaxy_map = expand_universe(galaxy_map);
    let universe = enumerate_universe(&galaxy_map);
    let unique_galaxies = get_unique_pairs(get_galaxy_count(&universe));
    let shortes_path_sum: usize = get_all_galaxies(&universe).into_iter()
        .map(|x| get_distances_to_galaxies(x, &universe))
        .flatten()
        .filter(|(x, _)| unique_galaxies.contains(x))
        .map(|(_,d)| d as usize)
        .sum();
    println!("shortes paths sum: {}", shortes_path_sum);
    draw_universe(&universe);
}

fn get_galaxy_count(universe: &Vec<Vec<UniverseNode>>) -> usize {
    get_cartesian_product_range(0..universe.len(), 0..universe[0].len())
        .into_iter()
        .filter(|(y,x)| {
            match universe[*y][*x] {
                 UniverseNode::EmptySpace => false,
                 UniverseNode::Galaxy(_) => true
            }
        })
    .count() 
}

fn get_distances_to_galaxies(start: (usize,usize),
                             universe: &Vec<Vec<UniverseNode>>) 
        -> Vec<((usize,usize),u16)> {
    let mut distances: HashMap<(usize,usize), u16> = HashMap::from_iter(
        (0..universe.len()).map(|y| (0..universe[0].len()).map(move |x| (y,x)))
            .flatten().map(|x| (x,u16::MAX))
    );
    *distances.get_mut(&start).unwrap() = 0;
    let mut to_be_visited: HashSet<(usize,usize)> = HashSet::from_iter(
        get_cartesian_product_range(0..universe.len(), 0..universe[0].len())
    );

    let mut current_node;
    
    while !to_be_visited.is_empty() {
        current_node = *to_be_visited.iter()
            .map(|x| (x, distances[x]))
            .min_by_key(|(_,d)| *d)
            .unwrap()
            .0;

        to_be_visited.remove(&current_node);
        for neigbor in get_neighbors(current_node, &universe) {
            if !to_be_visited.contains(&neigbor) {
                continue;
            }
            let current_distance = distances[&neigbor];
            let new_distance = distances[&current_node] + 1;
            if current_distance > new_distance {
                *distances.get_mut(&neigbor).unwrap() = new_distance;
            }
        }
    }

    draw_universe_with_distance(&distances, universe);
    println!("-------------------------------------------");
    println!("-------------------------------------------");
    println!("-------------------------------------------");
    get_all_galaxies(universe)
        .into_iter()
        .filter(|&x| x != start)
        .map(|(y,x)| ((y,x), distances[&(y,x)]))
        .collect()
}

fn draw_universe_with_distance(distance: &HashMap<(usize,usize), u16>, 
                               universe: &Vec<Vec<UniverseNode>>) {
    for row in 0..universe.len() {
        for column in 0..universe[0].len() {
            if distance.contains_key(&(row,column)) {
                print!("({})", distance[&(row,column)].to_string());
            } else {
                match universe[row][column] {
                    UniverseNode::EmptySpace => print!("."),
                    UniverseNode::Galaxy(_) => print!("#"),
                }
            }
        }
        println!()
    }
}

fn get_cartesian_product_range(x: Range<usize>, y: Range<usize>)
    -> Vec<(usize,usize)> {
    x.map(|row| y.clone().map(move |column| (row,column)))
        .flatten()
        .collect()
}

fn get_all_galaxies(universe: &Vec<Vec<UniverseNode>>) -> Vec<(usize,usize)> {
    get_cartesian_product_range(0..universe.len(), 0..universe[0].len())
        .into_iter()
        .filter(|(y,x)| {
            match universe[*y][*x] {
                 UniverseNode::EmptySpace => false,
                 UniverseNode::Galaxy(_) => true
            }
        })
    .collect()
}

fn get_neighbors(node: (usize,usize), universe: &Vec<Vec<UniverseNode>>) 
    -> Vec<(usize, usize)> {
    let neighbors = vec![
        (node.0 as i64, node.1 as i64 + 1),
        (node.0 as i64, node.1 as i64 - 1),
        (node.0 as i64 - 1, node.1 as i64),
        (node.0 as i64 + 1, node.1 as i64)
    ];
    neighbors.into_iter()
        .filter(|(y,x)| 
            (0..universe.len() as i64).contains(y) 
            && (0..universe[0].len() as i64).contains(x)
        )
        .map(|(y,x)| (y as usize, x as usize))
        .collect()
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
    let cartesian_product = (1..=amount)
        .map(|row| (1..=amount).map(move |column| (row,column)))
        .flatten()
        .collect::<Vec<(usize,usize)>>();
    let mut unique_pairs = vec![];
    for (x,y) in cartesian_product {
        if !unique_pairs.contains(&(y,x)) && x != y {
            unique_pairs.push((x,y))
        }
    }
    unique_pairs
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

        let universe = enumerate_universe(&expand_universe(galaxy));
        let distances = get_distances_to_galaxies((6,1), &universe);
        assert_eq!(get_unique_pairs(get_galaxy_count(&universe)).len(), 36);
        println!("distances: {:?}", distances);
        assert_eq!(
            distances[
             distances.iter()
                .position(|((y,x),_)| *y == 11 && *x == 5).unwrap()
            ].1,
            9
            );


        let unique_galaxies = get_unique_pairs(get_galaxy_count(&universe));
        println!("hallo: {:?}", unique_galaxies);
        let shortes_path_sum: usize = get_all_galaxies(&universe).into_iter()
            .map(|x| get_distances_to_galaxies(x, &universe))
            .flatten()
            .filter(|(x, _)| unique_galaxies.contains(x))
            .inspect(|(_,d)| println!("current distance: {}", d))
            .map(|(_,d)| d as usize)
            .sum();
        assert_eq!(shortes_path_sum, 374);
    }
}
