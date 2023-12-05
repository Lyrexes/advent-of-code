use std::collections::HashMap;

#[derive(Debug,Clone)]
struct Scratchcard {
    id: u8,
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>
}

fn main() {
    let scratchcards = include_str!("scratchcards.input")
        .lines()
        .map(parse_scratchcard)
        .collect::<Vec<Scratchcard>>();
    
    let sum_points: u64 = scratchcards.iter()
        .map(get_winning_points)
        .sum();
    println!("sum points: {}", sum_points);
    
    let won_cards = get_won_sratch_cards(&scratchcards);
    println!("won cards: {}", won_cards);
    
}

fn get_won_sratch_cards(scratchcards: &Vec<Scratchcard>) -> u128{
    let mut scratch_count: HashMap<u8, u128> = HashMap::from_iter(
        scratchcards.iter().map(|x| (x.id, 1u128))
    );
    for current_id in 1..=scratchcards.len() {
        let score = get_matching_numbers(&scratchcards[current_id - 1]);
        let current_scratchcard_count 
            = *scratch_count.get(&(current_id as u8)).unwrap();

        let end_id = usize::min(current_id + score as usize, scratchcards.len());
        for id in current_id+1..=end_id {
            *scratch_count.get_mut(&(id as u8)).unwrap() 
                += current_scratchcard_count;
        }
    }
    scratch_count.iter().map(|(_,x)| x).sum()
}

fn get_winning_points(scratchcard: &Scratchcard) -> u64 {
    let mut points = 0;

    for element in scratchcard.numbers.iter() {
        if scratchcard.winning_numbers.contains(&element) {
            if points == 0 {
                points = 1;
            } else {
                points = points << 1;
            }
        }
    }

    points
}

fn get_matching_numbers(scratchcard: &Scratchcard) -> u64 {
    let mut points = 0;

    for element in scratchcard.numbers.iter() {
        if scratchcard.winning_numbers.contains(&element) {
            points += 1;
        }
    }

    points
}

fn parse_scratchcard(mut raw: &str) -> Scratchcard {
    raw = raw.strip_prefix("Card ").unwrap();

    let (mut id_str, values) = raw.split_once(":").unwrap();
    id_str = id_str.trim();

    let (mut winning, mut yours) = values.split_once("|").unwrap();
    winning = winning.trim();
    yours   = yours.trim();

    let id = id_str.parse::<u8>().unwrap();

    let winning_numbers = winning.split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let numbers = yours.split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
        
    Scratchcard {
        id,
        winning_numbers,
        numbers
    }
}
