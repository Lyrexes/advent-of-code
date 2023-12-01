#[derive(Debug, Clone)]
struct Sensor {
    position: (i128, i128),
    nearest_beacon_position: (i128, i128),
}

fn main() {
    beacon_cant_be();
    get_tuning_frequency();
}

fn get_tuning_frequency() {
    let sensors = include_str!("scan")
        .lines()
        .map(parse_sensor)
        .collect::<Vec<Sensor>>();

    let mut current_position;
    let mut x;
    let max = 4000000i128;
    //let max = 20;
    for y in 0..=max {
        x = 0;
        while x <= max {
            current_position = (x as i128, y as i128);
            if let Some(sensor_index) = sensors
                .iter()
                .position(|x| is_in_known_area(x, current_position))
            {
                x += distance_to_border(
                    &sensors[sensor_index],
                    current_position,
                ) + 1;
            } else {
                println!(
                    "tuning-frequency: {}",
                    current_position.0 * max + current_position.1
                );
                return;
            }
        }
    }
}

fn distance_to_border(sensor: &Sensor, position: (i128, i128)) -> i128 {
    2 * sensor.position.0.abs_diff(position.0) as i128
        + ((sensor.position.0
            - get_manhattan_distance(
                sensor.position,
                sensor.nearest_beacon_position,
            ) as i128)
            + sensor.position.1.abs_diff(position.1) as i128)
            .abs_diff(position.0) as i128
}

fn is_in_known_area(sensor: &Sensor, position: (i128, i128)) -> bool {
    get_manhattan_distance(sensor.position, position)
        <= get_manhattan_distance(
            sensor.position,
            sensor.nearest_beacon_position,
        )
}

fn get_manhattan_distance(v: (i128, i128), u: (i128, i128)) -> i128 {
    (v.0 - u.0).checked_abs().unwrap() + (v.1 - u.1).checked_abs().unwrap()
}

fn beacon_cant_be() {
    let sensors = include_str!("scan")
        .lines()
        .map(parse_sensor)
        .collect::<Vec<Sensor>>();
    let searching_row = 2000000i128;
    let min_x = sensors
        .iter()
        .map(|x| {
            x.position.0
                - get_manhattan_distance(x.nearest_beacon_position, x.position)
                + x.position.1.abs_diff(searching_row) as i128
        })
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|x| {
            x.position.0
                + get_manhattan_distance(x.nearest_beacon_position, x.position)
                - x.position.1.abs_diff(searching_row) as i128
        })
        .max()
        .unwrap();

    let mut current_position;
    let mut beacon_cant_be_count = 0;
    for x in min_x..=max_x {
        current_position = (x as i128, searching_row);
        for sensor in sensors.iter() {
            if is_in_known_area(sensor, current_position)
                && sensor.nearest_beacon_position != current_position
            {
                beacon_cant_be_count += 1;
                break;
            }
        }
    }

    println!("{:?}", beacon_cant_be_count);
}

fn parse_sensor(raw: &str) -> Sensor {
    let positions = raw
        .split_ascii_whitespace()
        .filter(|x| x.starts_with("x=") || x.starts_with("y="))
        .map(|x| {
            x.trim_matches(|x| {
                x == '=' || x == 'x' || x == 'y' || x == ',' || x == ':'
            })
        })
        .map(|x| x.parse::<i128>())
        .flatten()
        .collect::<Vec<i128>>();

    Sensor {
        position: (positions[0], positions[1]),
        nearest_beacon_position: (positions[2], positions[3]),
    }
}
