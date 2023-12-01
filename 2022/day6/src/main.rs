use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let data_steam_file = File::open("datastream")?;
    let reader = BufReader::new(data_steam_file);
    let datastream = reader
        .lines()
        .into_iter()
        .flatten()
        .collect::<String>();

    println!("packet marker: {}", packet_marker(&datastream));
    println!("message marker: {}", message_marker(&datastream));
    Ok(())
}

fn message_marker(data: &String) -> usize {
    let datastream_length = data.len();
    let mut current_slice;
    let mut received = 0;
    for i in 0..datastream_length {
        if i + 13 >= datastream_length {
            break;
        }
        current_slice = &data.as_str()[i..i+14];
        if no_duplicate(current_slice) {
            received = i+14;
            break;
        }
    }
    received
}

fn packet_marker(data: &String) -> usize {
    let datastream_length = data.len();
    let mut current_slice;
    let mut received = 0;
    for i in 0..datastream_length {
        if i + 3 >= datastream_length {
            break;
        }
        current_slice = &data.as_str()[i..i+4];
        if no_duplicate(current_slice) {
            received = i+4;
            break;
        }
    }
    received
}

fn no_duplicate(slice: &str)-> bool{
    for i in 0..slice.len() {
        if slice[i+1..].contains(slice.chars().nth(i).unwrap()){
            return false;
        }
    }
    return true;
}


