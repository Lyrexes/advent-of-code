
fn main() {
    let calibration = include_str!("calibration");
    let calibration_instruction : u32 = calibration.lines()
        .map(|x| x.to_string())
        .map(filter_alphanum)
        .map(first_last)
        .map(|x| x.parse::<u32>().unwrap())
        .sum();
    println!("{:?}", calibration_instruction);

    let calibration2 = include_str!("calibration");
    let calibration_instruction2 : u32 = calibration2.lines()
        .map(|x| x.to_string())
        .map(num_string_to_num)
        .map(filter_alphanum)
        .map(first_last)
        .map(|x| x.parse::<u32>().unwrap())
        .sum();
    println!("{:?}", calibration_instruction2)
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

fn num_string_to_num(mut str: String) -> String {
    let num_as_strings = 
        ["one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine"];
    for (index, num_str) in num_as_strings.iter().enumerate() {
        println!("{}", index+1);
        str = str.replace(num_str, (index + 1).to_string().as_str());
    }
    return str
}

