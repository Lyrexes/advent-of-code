use std::cmp::Ordering;

struct One;
#[derive(Debug)]
struct Two;

#[derive(Debug,Clone)]
struct Player <Part = One>{
    hand: Vec<u8>,
    bid: u16,
    part: std::marker::PhantomData<Part>
}

impl From<&str> for Player<One> {
    fn from(raw: &str) -> Self {
        let (hand_raw, bid_raw) = raw.split_once(" ").unwrap();
        let hand = hand_raw.chars()
                .map(|x| {
                    match x {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => 11,
                        'T' => 10,
                        _   => x.to_digit(10).unwrap() as u8
                    }
                })
                .collect();
        let bid = bid_raw.parse::<u16>().unwrap();
        Player::<One> {
            hand,
            bid,
            part: std::marker::PhantomData
        }
    }
}

impl From<&str> for Player<Two> {
    fn from(raw: &str) -> Self {
        let (hand_raw, bid_raw) = raw.split_once(" ").unwrap();
        let hand = hand_raw.chars()
                .map(|x| {
                    match x {
                        'A' => 13,
                        'K' => 12,
                        'Q' => 11,
                        'T' => 10,
                        'J' => 1,
                        _   => x.to_digit(10).unwrap() as u8
                    }
                })
                .collect();
        let bid = bid_raw.parse::<u16>().unwrap();
        Player::<Two> {
            hand,
            bid,
            part: std::marker::PhantomData
        }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Player<One> {}

impl Ord for Player<One> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_score = get_score(&self.hand);
        let other_score = get_score(&other.hand);
        if self_score == other_score {
            self.hand.cmp(&other.hand)
        } else {
            self_score.cmp(&other_score)
        }
    }
}

impl PartialOrd for Player<One> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_score = get_score(&self.hand);
        let other_score = get_score(&other.hand);
        if self_score == other_score {
            Some(self.hand.cmp(&other.hand))
        } else {
            Some(self_score.cmp(&other_score))
        }
    }
}

impl PartialEq for Player<Two> {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Player<Two> {}

impl Ord for Player<Two> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_score = get_score_joker(&self.hand);
        let other_score = get_score_joker(&other.hand);
        if self_score == other_score {
            self.hand.cmp(&other.hand)
        } else {
            self_score.cmp(&other_score)
        }
    }
}

impl PartialOrd for Player<Two> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_score = get_score_joker(&self.hand);
        let other_score = get_score_joker(&other.hand);
        if self_score == other_score {
            Some(self.hand.cmp(&other.hand))
        } else {
            Some(self_score.cmp(&other_score))
        }
    }
}

fn get_score_joker(hand: &Vec<u8>) -> u8 {
    let mut card_counts = (2..=13)
        .map(|c| hand.iter().filter(|&x| *x == c ).count())
        .map(|x| x as u8)
        .collect::<Vec<u8>>();
    let joker_count = hand.iter().filter(|&x| *x == 1).count();
    let max_card_count = *card_counts.iter().max().unwrap();
    let max_card_count_index = card_counts.iter()
        .position(|&x| x == max_card_count)
        .unwrap();
    card_counts.remove(max_card_count_index);
    match max_card_count + joker_count as u8 {
        5 => 6,
        4 => 5,
        3 => {
            if card_counts.contains(&2u8){
                4
            } else {
                3
            }
        }
        2 => {
            if card_counts.contains(&2u8) {
                2
            } else {
                1
            }
        }
        _ => 0
    }
}

fn get_score(hand: &Vec<u8>) -> u8 {
    let card_counts = (1..=14)
        .map(|c| hand.iter().filter(|&x| *x == c ).count())
        .map(|x| x as u8)
        .collect::<Vec<u8>>();
    match card_counts.iter().max().unwrap() {
        5 => 6,
        4 => 5,
        3 => {
            if card_counts.contains(&2u8) {
                4
            } else {
                3
            }
        }
        2 => {
            if card_counts.iter().filter(|&x| *x == 2 ).count() == 2 {
                2
            } else {
                1
            }
        }
        _ => 0
    }
}

fn main() {
    let mut players = include_str!("camel-cards.input").split("\n")
        .filter(|x| !x.is_empty())
        .map(Player::<One>::from)
        .collect::<Vec<Player>>();
    players.sort();
    let total_winnings: u64 = players.iter()
        .enumerate()
        .map(|(i,x)| (i as u64 + 1) * x.bid as u64)
        .sum();
    println!("total winnings: {}", total_winnings);

    let mut players_joker = include_str!("camel-cards.input").split("\n")
        .filter(|x| !x.is_empty())
        .map(Player::<Two>::from)
        .collect::<Vec<Player::<Two>>>();
    players_joker.sort();
    let total_winnings_joker: u64 = players_joker.iter()
        .enumerate()
        .map(|(i,x)| (i as u64 + 1) * x.bid as u64)
        .sum();

    println!("total winnings joker rule: {}", total_winnings_joker);
}
