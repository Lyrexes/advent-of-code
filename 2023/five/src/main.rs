use std::{ops::RangeInclusive, rc::Rc};

#[derive(Debug)]
struct Conversion {
    source: RangeInclusive<u128>,
    destination: RangeInclusive<u128>
}

#[derive(Debug)]
struct Almanac {
    seeds: Box<[u128]>,
    converters: Box<[Box<[Conversion]>]>,
}

#[derive(Debug)]
struct AlmanacPartTwo {
    seeds: Box<[RangeInclusive<u128>]>,
    converters: Box<[Box<[Conversion]>]>,
}


fn main() {
    let almanac = parse_almanac(include_str!("almanac.input"));
    
    let min_location = almanac.seeds.iter()
        .map(|&x| convert_to_last(x, &almanac.converters))
        .min()
        .unwrap();

    let mut almanac_part_two = almanac_to_alamanac_part_two(almanac);
    println!("min location: {}", min_location);

    let min_location_ranges = almanac_part_two.seeds.iter()
        .map(|x| convert_range_to_last(x, &mut almanac_part_two.converters))
        .flatten()
        .map(|x| *x.start())
        .min()
        .unwrap();

    println!("min location ranges: {}", min_location_ranges);
}


fn almanac_to_alamanac_part_two(old: Almanac) -> AlmanacPartTwo {
    let mut seed_ranges = vec![];
    for i in (0..old.seeds.len()).step_by(2) {
        seed_ranges.push(
            old.seeds[i]..=old.seeds[i] + old.seeds[i+1] -1
        );
    }

    AlmanacPartTwo {
        seeds: seed_ranges.into(),
        converters: old.converters,
    }
}

fn parse_almanac(raw: &str) -> Almanac{
    let alamanac_raw = raw.split("\n\n").collect::<Rc<[&str]>>();
    let seeds = alamanac_raw[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Box<[u128]>>();

    let mut converters = vec![];
    for index in 1..alamanac_raw.len() {
        let (_, converter_raw) = alamanac_raw[index].split_once("\n").unwrap();
        let converter = converter_raw.split("\n")
            .filter(|x| !x.is_empty())
            .map(parse_conversion)
            .collect::<Box<[Conversion]>>();
        converters.push(converter);
    }

    Almanac {
        seeds,
        converters: converters.into()
    }
}

fn parse_conversion(raw: &str) -> Conversion {
    let numbers = raw.split(" ")
        .filter(|x| !str::is_empty(x))
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


fn convert(source_value: u128, conversions: &Box<[Conversion]>) -> u128 {
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

fn convert_range_one(source_range: RangeInclusive<u128>, 
                     conversions: &mut Box<[Conversion]>)
    -> (RangeInclusive<u128>, u128) {

    conversions.sort_by(|x,y| x.source.start().cmp(y.source.start()));

    for conversion in conversions.iter() {
        if is_range_inside_other(source_range.clone(), conversion.source.clone()) {
            let range_length = u128::abs_diff (
                *conversion.source.start(),
                *source_range.start()
            );
            
            let start = *source_range.start();
            let end   = *conversion.source.start() - 1;
            return (start..=end, source_range.start() + range_length);
        }
    }


    
    for conversion in conversions.iter() {
        if conversion.source.contains(&source_range.start()) {
            let delta_start = u128::abs_diff(
                *conversion.source.start(),
                *source_range.start()
            );

            let start = conversion.destination.start() + delta_start;

            let delta_end = u128::min (
                u128::abs_diff (
                    *conversion.source.end(),
                    *source_range.start()
                ),
                u128::abs_diff(
                    *source_range.start(),
                    *source_range.end()
                )
            );

            let end   = start + delta_end;
            return (start..=end, source_range.start() + delta_end + 1);
        }
    }

    for conversion in conversions.iter() {
        if conversion.source.contains(&source_range.end()) {
            let range_length = u128::abs_diff (
                *conversion.source.start(),
                *source_range.start()
            );
            
            let start = *source_range.start();
            let end   = *conversion.source.start() - 1;
            return (start..=end, source_range.start() + range_length);
        }
    }


    (source_range.clone(), *source_range.end())
}

fn convert_range(source_range: RangeInclusive<u128>,
                 conversions: &mut Box<[Conversion]>) -> Vec<RangeInclusive<u128>> {
    let mut ranges = vec![];
    let mut current_source_start = *source_range.start();
    let mut current_range = source_range.clone();
    while current_source_start < *source_range.end() {
        (current_range, current_source_start) = convert_range_one(
            current_range.clone(),
            conversions
        );
        ranges.push(current_range.clone());
        current_range = current_source_start..=*source_range.end();
    }
    ranges
}

fn is_range_inside_other(big: RangeInclusive<u128>, 
                         small: RangeInclusive<u128>) -> bool {
    big.start() < small.start() && big.end() > small.end()
}

fn convert_to_last(source_value: u128, converters: &Box<[Box<[Conversion]>]>) 
    -> u128 {
    let mut converted_value = source_value;
    for converter in converters.iter() {
        converted_value = convert(converted_value, converter);
    }
    converted_value
}

fn convert_range_to_last(source_range: &RangeInclusive<u128>,
                         converters: &mut Box<[Box<[Conversion]>]>) 
    -> Vec<RangeInclusive<u128>>{
    let mut converted_ranges = vec![source_range.clone()];
    let mut current_ranges = vec![];
    for converter in converters.iter_mut() {
        for range in converted_ranges.iter() {
            current_ranges.extend(convert_range(range.clone(), converter));
        }
        converted_ranges = current_ranges;
        current_ranges = vec![];
    }
    converted_ranges
}

