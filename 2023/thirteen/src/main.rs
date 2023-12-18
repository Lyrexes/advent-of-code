
fn main() {
    let lava_ashes = include_str!("reflections.input")
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split("\n")
             .filter(|x| !x.is_empty())
             .map(|x| x.bytes().collect())
             .collect::<Vec<Vec<u8>>>())
        .collect::<Box<[Vec<Vec<u8>>]>>();

    let sum_row: usize = lava_ashes.iter()
        .map(|x| get_reflection_index_row(x))
        .flatten()
        .map(|x| x * 100)
        .sum();

    let sum_column: usize = lava_ashes.iter()
        .map(|x| get_reflection_index_column(x))
        .flatten()
        .sum();

    println!("sum notes: {}", sum_row + sum_column);

    let sum_row_smudge: usize = lava_ashes.iter()
        .map(|x| get_reflection_index_row_smudge(x))
        .flatten()
        .map(|x| x * 100)
        .sum();


    let sum_column_smudge: usize = lava_ashes.iter()
        .map(|x| get_reflection_index_column_smudge(x))
        .flatten()
        .sum();

    println!("sum notes smudges: {}", sum_row_smudge + sum_column_smudge);
}

fn get_reflection_candidates_row(lava_ash: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut candidates = vec![];
    for (i,row_pair) in lava_ash.windows(2).enumerate(){
        if row_pair[0] == row_pair[1] {
            candidates.push(i+1);
        }
    }
    candidates
}

fn is_reflecting_row(lava_ash: &Vec<Vec<u8>>, candidate: usize) -> bool {
    for reflection in 1..candidate {
        let reflection_below = candidate - (reflection+1);
        let reflection_above = candidate + reflection;
        if !(0..lava_ash.len()).contains(&reflection_above) 
                || !(0..lava_ash.len()).contains(&reflection_below) {
            continue;
        }
        if lava_ash[reflection_below] != lava_ash[reflection_above] {
            return false
        }
    }
    true
}

fn get_reflection_index_row(lava_ash: &Vec<Vec<u8>>) -> Option<usize> {
    let index_candidates = get_reflection_candidates_row(lava_ash);
    for index in index_candidates {
        if is_reflecting_row(lava_ash, index) {
            return Some(index)
        }
    }
    None
}

fn get_reflection_candidates_column(lava_ash: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut candidates = vec![];
    for column in 0..lava_ash[0].len() - 1{
        if are_columns_equal(lava_ash, column, column + 1) {
            candidates.push(column + 1);
        }
    }
   candidates
}

fn is_reflecting_column(lava_ash: &Vec<Vec<u8>>, candidate: usize) -> bool {
    for reflection in 1..candidate {
        let reflection_below = candidate - (reflection+1);
        let reflection_above = candidate + reflection;
        if !(0..lava_ash[0].len()).contains(&reflection_above) 
                || !(0..lava_ash[0].len()).contains(&reflection_below) {
            continue;
        }
        if !are_columns_equal(lava_ash, reflection_below, reflection_above) {
            return false
        }
    }
    true
}

fn get_reflection_index_column(lava_ash: &Vec<Vec<u8>>) -> Option<usize> {
    let index_candidates = get_reflection_candidates_column(lava_ash);
    for index in index_candidates {
        if is_reflecting_column(lava_ash, index) {
            return Some(index)
        }
    }
    None
}

fn are_columns_equal(lava_ash: &Vec<Vec<u8>>, i: usize, j: usize) 
        -> bool {
    for row in 0..lava_ash.len() {
        if lava_ash[row][i] != lava_ash[row][j] {
            return false
        }
    }
    true
}

fn column_diffrent_count(lava_ash: &Vec<Vec<u8>>, i: usize, j: usize) -> usize {
    let mut count = 0;
    for row in 0..lava_ash.len() {
        if lava_ash[row][i] != lava_ash[row][j] {
            count += 1
        }
    }
    count
}

fn row_diffrent_count(lava_ash: &Vec<Vec<u8>>, i: usize, j: usize) -> usize {
    let mut count = 0;
    for column in 0..lava_ash[0].len() {
        if lava_ash[i][column] != lava_ash[j][column] {
            count += 1
        }
    }
    count
}

fn reflection_candidates_row_smudge(lava_ash: &Vec<Vec<u8>>) 
    -> Vec<(usize,usize)>{
    let mut candidates = vec![];
    for row in 0..lava_ash.len() - 1 {
        let diffrent_count = row_diffrent_count(lava_ash, row, row + 1);
        if diffrent_count <= 1 {
            candidates.push((row + 1, diffrent_count));
        }
    }
   candidates
}

fn get_reflection_index_row_smudge(lava_ash: &Vec<Vec<u8>>) -> Option<usize> {
    let index_candidates = reflection_candidates_row_smudge(lava_ash);
    for (index,count) in index_candidates {
        if is_reflecting_row_smudge(lava_ash, index, count) {
            return Some(index)
        }
    }
    None
}

fn is_reflecting_row_smudge(lava_ash: &Vec<Vec<u8>>, candidate: usize, 
                            count: usize) -> bool {
    let mut diffrent_count = count;
    for reflection in 1..candidate {
        let reflection_below = candidate - (reflection+1);
        let reflection_above = candidate + reflection;
        if !(0..lava_ash.len()).contains(&reflection_above) 
                || !(0..lava_ash.len()).contains(&reflection_below) {
            continue;
        }
        let current_diffrent_count = row_diffrent_count(
                lava_ash,
                reflection_below,
                reflection_above
        );
        diffrent_count += current_diffrent_count;
        if diffrent_count > 1{
            return false
        }
    }
    diffrent_count == 1
}

fn get_reflection_index_column_smudge(lava_ash: &Vec<Vec<u8>>) -> Option<usize> {
    let index_candidates = reflection_candidates_column_smudge(lava_ash);
    for (index,count) in index_candidates {
        if is_reflecting_column_smudge(lava_ash, index, count) {
            return Some(index)
        }
    }
    None
}

fn is_reflecting_column_smudge(lava_ash: &Vec<Vec<u8>>, candidate: usize, 
                               count: usize) -> bool {
    let mut diffrent_count = count;
    for reflection in 1..candidate {
        let reflection_below = candidate - (reflection+1);
        let reflection_above = candidate + reflection;
        if !(0..lava_ash[0].len()).contains(&reflection_above) 
                || !(0..lava_ash[0].len()).contains(&reflection_below) {
            continue;
        }
        let current_diffrent_count = column_diffrent_count(
                lava_ash,
                reflection_below,
                reflection_above
        );
        diffrent_count += current_diffrent_count;
        if diffrent_count > 1{
            return false
        }
    }
    diffrent_count == 1
}

fn reflection_candidates_column_smudge(lava_ash: &Vec<Vec<u8>>) 
    -> Vec<(usize,usize)> {
    let mut candidates = vec![];
    for column in 0..lava_ash[0].len() - 1 {
        let diffrent_count = column_diffrent_count(
            lava_ash,
            column,
            column + 1
        );
        if diffrent_count <= 1 {
            candidates.push((column + 1, diffrent_count));
        }
    }
   candidates
}
