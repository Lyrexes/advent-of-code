#[derive(Debug)]
struct Round {
    red: u8,
    blue: u8,
    green: u8,
}

#[derive(Debug)]
struct Game {
    id: u8,
    rounds: Vec<Round>,
}

fn main() {
    let games_input = include_str!("elve-game.input");
    let games = games_input.lines().map(parse_game).collect::<Vec<Game>>();

    let possible_sum: u32 = games.iter()
        .filter(is_possible)
        .map(|x| x.id as u32)
        .sum();
    println!("{}", possible_sum);

    let minimu_possible_sum: u32 = games.iter()
        .map(get_minimum_possible_round)
        .map(|x| x.red as u32 * x.green as u32 * x.blue as u32)
        .sum();
    println!("{}", minimu_possible_sum)
}

fn get_minimum_possible_round(game: &Game) -> Round {
    let red_max   = game.rounds.iter().map(|x| x.red).max().unwrap();
    let green_max = game.rounds.iter().map(|x| x.green).max().unwrap();
    let blue_max  = game.rounds.iter().map(|x| x.blue).max().unwrap();

    Round {
        red: red_max,
        blue: blue_max,
        green: green_max
    }
}

fn is_possible(game: &&Game) -> bool {
    let red_max   = 12u8;
    let green_max = 13u8;
    let blue_max  = 14u8;
    let mut is_possible = true;

    for round in game.rounds.iter() {
       is_possible = is_possible
           && round.red <= red_max 
           && round.green <= green_max 
           && round.blue <= blue_max
    }
    is_possible   
}

fn parse_game(mut game_str: &str) -> Game {
    game_str = game_str.strip_prefix("Game ").unwrap();

    let (id_str, rounds_str) = game_str.split_once(":").unwrap();
    let id = id_str.parse::<u8>().unwrap();
    let rounds = rounds_str
        .split(";")
        .map(parse_round)
        .collect::<Vec<Round>>();

    Game { id, rounds }
}

fn parse_round(round_str: &str) -> Round {
    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0
    };

    for color_str in round_str.split(",") {
        let (count_str, color) = color_str.trim().split_once(" ").unwrap();
        let count = count_str.parse::<u8>().unwrap();
        match color {
            "blue"  => round.blue = count,
            "red"   => round.red  = count,
            "green" => round.green = count,
            _ => {}
        }
    }
    round
}
