use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
enum ShellType {
    Directory(String),
    File(u32),
}

fn main() -> io::Result<()> {
    let history = BufReader::new(File::open("history")?)
        .lines()
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();

    let file_system = parse_directories(history);
    let diretory_sizes = evaluate_direcotry_sizes(&file_system);

    println!(
        "sum of all directories wiht size at most 100000: {}",
        get_sum_of_small_size(&diretory_sizes)
    );
    println!(
        "size of smalles directory to free up enough for the update: {}",
        smallest_directory_free_up_enough(&diretory_sizes)
    );
    Ok(())
}

fn smallest_directory_free_up_enough(directory_sizes: &HashMap<String, u32>) -> u32 {
    let disk_space = 70000000u32;
    let space_used = directory_sizes.get("/").unwrap();
    let space_update = 30000000u32;
    let space_needed = space_used + space_update - disk_space;
    let mut min_size = u32::MAX;
    for directory_size in directory_sizes.values() {
        if *directory_size >= space_needed && *directory_size < min_size {
            min_size = *directory_size;
        }
    }
    min_size
}

fn get_sum_of_small_size(directory_sizes: &HashMap<String, u32>) -> u32 {
    let mut sum = 0u32;
    for directory_size in directory_sizes.values() {
        if *directory_size <= 100000u32 {
            sum += directory_size;
        }
    }
    sum
}

fn evaluate_direcotry_sizes(file_system: &HashMap<String, Vec<ShellType>>) -> HashMap<String, u32> {
    let mut directory_sizes = HashMap::new();
    for current_paht in file_system.keys() {
        directory_sizes.insert(
            current_paht.to_string(),
            evaluate_direcotry_size(current_paht, file_system),
        );
    }
    directory_sizes
}

fn evaluate_direcotry_size(
    current_path: &String,
    file_system: &HashMap<String, Vec<ShellType>>,
) -> u32 {
    let mut directory_size = 0;
    let mut new_path;
    for shell_object in file_system.get(current_path).unwrap() {
        match shell_object {
            ShellType::File(size) => directory_size += size,
            ShellType::Directory(new_directory) => {
                new_path = current_path.clone();
                change_directory_forward(&mut new_path, new_directory);
                directory_size += evaluate_direcotry_size(&new_path, file_system);
            }
        }
    }
    directory_size
}

fn parse_directories(history: Vec<String>) -> HashMap<String, Vec<ShellType>> {
    let mut current_path = "".to_string();
    let mut file_system = HashMap::new();
    for line in history {
        if line.starts_with('$') {
            execute_command(line, &mut current_path);
        } else if line.starts_with("dir") {
            insert_dir(line, &current_path, &mut file_system)
        } else {
            insert_file(line, &current_path, &mut file_system)
        }
    }
    file_system
}

fn insert_file(
    line: String,
    current_path: &String,
    file_system: &mut HashMap<String, Vec<ShellType>>,
) {
    let new_file = line
        .split_ascii_whitespace()
        .nth(0)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    if file_system.contains_key(current_path) {
        file_system
            .get_mut(current_path)
            .unwrap()
            .push(ShellType::File(new_file));
    } else {
        file_system.insert(current_path.to_string(), vec![ShellType::File(new_file)]);
    }
}

fn insert_dir(
    line: String,
    current_path: &String,
    file_system: &mut HashMap<String, Vec<ShellType>>,
) {
    let new_directory = line
        .split_ascii_whitespace()
        .nth_back(0)
        .unwrap()
        .to_string();

    if file_system.contains_key(current_path) {
        file_system
            .get_mut(current_path)
            .unwrap()
            .push(ShellType::Directory(new_directory));
    } else {
        file_system.insert(
            current_path.to_string(),
            vec![ShellType::Directory(new_directory)],
        );
    }
}

fn execute_command(cmd: String, current_path: &mut String) {
    let dir_to_change;
    if cmd.contains("cd") {
        if cmd.contains("..") {
            change_directory_backward(current_path);
        } else {
            dir_to_change = cmd.split_ascii_whitespace().nth(2).unwrap();
            change_directory_forward(current_path, dir_to_change);
        }
    }
}

fn change_directory_forward(path: &mut String, dir: &str) {
    if !path.is_empty() && path != "/" {
        path.push('/');
    }
    path.push_str(dir);
}

fn change_directory_backward(path: &mut String) {
    let (rest, _) = path.rsplit_once('/').unwrap();
    path.replace_range(rest.len().., "");
    if path.is_empty() {
        path.push('/');
    }
}
