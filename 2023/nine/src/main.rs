
fn main() {
    let report = parse_report(include_str!("report.input"));

    let interpolation_sum: i32  = report.iter()
        .map(interpolate_sequence)
        .sum();
    println!("{:?}", interpolation_sum);

    let interpolation_backwards_sum: i32  = report.iter()
        .map(interpolate_sequence_backwards)
        .sum();
    println!("{:?}", interpolation_backwards_sum);
}

fn interpolate_sequence_backwards(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|&x| x == 0) {
        return 0
    } 
    let delta_sequence = get_diffrence_vector(sequence);
    sequence.first().unwrap() - interpolate_sequence_backwards(&delta_sequence)
}

fn interpolate_sequence(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|&x| x == 0) {
        return 0
    } 
    let delta_sequence = get_diffrence_vector(sequence);
    sequence.last().unwrap() + interpolate_sequence(&delta_sequence)
}

fn get_diffrence_vector(sequence: &Vec<i32>) -> Vec<i32> {
    sequence.windows(2).map(|x| x[1] - x[0]).collect()
}

fn parse_report(raw: &str) -> Vec<Vec<i32>> {
    raw.split("\n").filter(|x| !x.is_empty())
        .map(parse_sequence)
        .collect()
}

fn parse_sequence(raw: &str) -> Vec<i32>{
    raw.split_ascii_whitespace()
       .filter(|x| !x.is_empty())
       .map(|x| x.parse::<i32>().unwrap())
       .collect()
}
