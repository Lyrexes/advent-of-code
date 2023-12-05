
fn main() {
    let calibration = include_str!("calibration.input");
    let calibration_instruction : u32 = calibration.lines()
        .map(|x| x.to_string())
        .map(filter_alphanum)
        .map(first_last)
        .map(|x| x.parse::<u32>().unwrap())
        .sum();
    println!("{:?}", calibration_instruction);

    let calibration_instruction2: u32 = calibration.lines()
        .map(|x| x.to_string())
        .map(num_string_to_num)
        .map(first_last)    
        .map(|x| x.parse::<u32>().unwrap())
        .sum();
        println!("{}", calibration_instruction2)
}

fn filter_alphanum(calib: String) -> String {
    calib.bytes()
        .filter(u8::is_ascii_digit)
        .map(|x| x as char)
        .collect::<String>()
}

fn first_last(str: String) -> String{
    format!(
        "{}{}",
        str.chars().nth(0).unwrap(),
        str.chars().nth_back(0).unwrap()
    )
}

fn num_string_to_num(str: String) -> String {
    let mut numbers = "".to_string();

    for (index, element) in str.bytes().enumerate() {
        if u8::is_ascii_digit(&element) {
            numbers.push(element as char);
            continue;
        }
        match str.get(index..index+3).unwrap_or("") {
            "one" => numbers.push('1'),
            "two" => numbers.push('2'),
            "six" => numbers.push('6'),
            _ => {}
        }
        
        match str.get(index..index+4).unwrap_or("") {
            "four" => numbers.push('4'),
            "five" => numbers.push('5'),
            "nine" => numbers.push('9'),
            _ => {}
        }

        match str.get(index..index+5).unwrap_or("") {
            "three" => numbers.push('3'),
            "seven" => numbers.push('7'),
            "eight" => numbers.push('8'),
            _ => {}
        }
    }

    return numbers
}

