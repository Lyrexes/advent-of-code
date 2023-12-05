use std::{ops::RangeInclusive, rc::Rc};


struct Conversion {
    source: RangeInclusive<u128>,
    destination: RangeInclusive<u128>
}

struct Almanac {
    seeds: Rc<[u128]>,
    converters: Rc<[Conversion]>,
}


fn main() {
    let almanac = parse_almanac(include_str!("almanac.input"));
    
    println!("{:?}", almanac);
}

fn parse_almanac(raw: &str) {
    let alamanac_raw = raw.split("\n\n")
        .collect::<Rc<[&str]>>();
    let seeds = alamanac_raw[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Rc<[u128]>>();
    let converters = vec![];
    for index in 1..alamanac_raw.len() {
        let (_, converter_raw) = alamanac_raw[index].split_once("\n").unwrap();
        let converter = convert_raw.split("\n")
            .map(parse_conversion)
            .collect::<Rc<[u8]>>();
        converters.push(converter);
    }


    println!("{:?}", seeds);
}

fn parse_conversion(raw: &str) -> Conversion {
    let numbers = raw.split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Rc<[u128]>>();
    let start_destination = numbers[0];
    let start_source      = numbers[1];
    let length            = numbers[2];
    Conversion {
        source: start_source..=start_source + length - 1,
        destination: start_destination..=start_destination + length - 1
    }
}


fn convert(source_value: u128, conversions: &Vec<Conversion>) -> u128 {
    for conversion in conversions.iter() {
        if conversion.source.contains(&source_value) {
            let delta = u128::abs_diff(
                *conversion.source.start(),
                source_value
            );
            return conversion.destination.start() + delta;
        }
    }
    source_value
}
