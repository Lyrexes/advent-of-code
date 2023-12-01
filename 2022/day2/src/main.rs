use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Clone, Copy)]
enum GameEnding {
    Win,
    Lose,
    Draw
}

fn main() -> io::Result<()> {
    let file = File::open("rps")?;
    let reader = BufReader::new(file);
    let mut chunks: Vec<u8>;
    let mut rps_first = vec![];
    let mut rps_second = vec![];
    for resultLine in reader.lines() {
        chunks = resultLine?
            .split_whitespace()
            .into_iter()
            .map(|x| x.bytes().nth(0).unwrap())
            .collect();
        rps_first.push((
            parse_enemy_side(chunks.get(0).unwrap().to_owned()),
            parse_my_side(chunks.get(1).unwrap().to_owned()),
        ));

        rps_second.push((
            parse_enemy_side(chunks.get(0).unwrap().to_owned()),
            parse_game_ending(chunks.get(1).unwrap().to_owned()),
        ));
    }
    println!("first strategy: {}", evalutae_first_strategy(rps_first));
    println!("second strategy: {}", evalutae_second_strategy(rps_second));
    Ok(())
}

fn evalutae_second_strategy(rps: Vec<(RPS, GameEnding)>) -> u32 {
    let mut score: u32 = 0;
    let mut my_move;
    for (enemy, ending) in rps {
        my_move = game_ending_to_response(ending, enemy);
        score += get_val(my_move);
        score += get_score(enemy, my_move);
    }
    score
}

fn evalutae_first_strategy(rps: Vec<(RPS, RPS)>) -> u32 {
    let mut score: u32 = 0;
    for (en, my) in rps {
        score += get_val(my);
        score += get_score(en, my);
    }
    score
}

fn get_score(enemy: RPS, myself: RPS) -> u32 {
    match (enemy, myself) {
        (RPS::Rock, RPS::Rock) => 3,
        (RPS::Paper, RPS::Paper) => 3,
        (RPS::Scissor, RPS::Scissor) => 3,
        (RPS::Rock, RPS::Paper) => 6,
        (RPS::Paper, RPS::Scissor) => 6,
        (RPS::Scissor, RPS::Rock) => 6,
        _ => 0,
    }
}

fn game_ending_to_response(ending: GameEnding, enemy: RPS) -> RPS {
    match ending {
        GameEnding::Win => get_response_win(enemy),
        GameEnding::Lose => get_response_lose(enemy),
        GameEnding::Draw => enemy,
    }
}

fn get_response_win(rps: RPS) -> RPS {
    match rps {
        RPS::Paper => RPS::Scissor,
        RPS::Scissor => RPS::Rock,
        RPS::Rock => RPS::Paper,
    }
}

fn get_response_lose(rps: RPS) -> RPS {
    match rps {
        RPS::Scissor => RPS::Paper,
        RPS::Rock => RPS::Scissor,
        RPS::Paper => RPS::Rock,
    }
}

fn get_val(rps: RPS) -> u32 {
    match rps {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissor => 3,
    }
}

fn parse_game_ending(rps: u8) -> GameEnding {
    match rps {
        b'X' => GameEnding::Lose,
        b'Y' => GameEnding::Draw,
        b'Z' => GameEnding::Win,
        _ => unreachable!(),
    }
}

fn parse_my_side(rps: u8) -> RPS {
    match rps {
        b'X' => RPS::Rock,
        b'Y' => RPS::Paper,
        b'Z' => RPS::Scissor,
        _ => unreachable!(),
    }
}

fn parse_enemy_side(rps: u8) -> RPS {
    match rps {
        b'A' => RPS::Rock,
        b'B' => RPS::Paper,
        b'C' => RPS::Scissor,
        _ => unreachable!(),
    }
}
