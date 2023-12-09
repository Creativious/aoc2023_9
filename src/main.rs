use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() {
    let mut part_1_total = 0;
    let mut part_2_total = 0;
    let file_lines = read_lines("puzzle_input.txt");
    for line in file_lines {
        let numbers = get_numbers_from_regex(&*line, Regex::new(r"(?m)(-?\d*)").unwrap());
        let guessed_value_part_1 = get_guessed_value_part_1(numbers.clone());
        let guessed_value_part_2 = get_guessed_value_part_2(numbers);
        part_1_total += guessed_value_part_1;
        part_2_total += guessed_value_part_2;
    }
    println!("The total is for part 1 {}", part_1_total);
    println!("The total is for part 2 {}", part_2_total);
}

fn get_guessed_value_part_1(input_numbers: Vec<i64>) -> i64{
    let mut numbers_list_maps = HashMap::new();
    let mut i = 1;
    numbers_list_maps.insert(0, input_numbers.clone());
    loop {
        let mut number_list = Vec::new();
        if i == 1 {
            number_list = get_difference_numbers_list(&input_numbers);
            numbers_list_maps.insert(i, number_list.clone());
        } else {
            number_list = get_difference_numbers_list(&numbers_list_maps.get(&(&i - 1)).unwrap());
            numbers_list_maps.insert(i, number_list.clone());
        }
        let mut all_zeros = true;
        for number in number_list {
            if number != 0 {
                all_zeros = false;
                break;
            }
        }
        if all_zeros {
            break;
        }
        i += 1;
    }
    let map_size = numbers_list_maps.keys().count();
    let start_index_backwards = map_size - 1;
    let mut x = start_index_backwards;
    let mut last_number = 0;
    loop {
        let mut number_list = numbers_list_maps.get(&x).unwrap();
        let list_size = number_list.iter().count();
        last_number = number_list.get(list_size - 1).unwrap() + last_number.clone();
        if x == 0 {
            break;
        }
        x -= 1;
    }
    last_number
}

fn get_guessed_value_part_2(input_numbers: Vec<i64>) -> i64{
    let mut numbers_list_maps = HashMap::new();
    let mut i = 1;
    numbers_list_maps.insert(0, input_numbers.clone());
    loop {
        let mut number_list = Vec::new();
        if i == 1 {
            number_list = get_difference_numbers_list(&input_numbers);
            numbers_list_maps.insert(i, number_list.clone());
        } else {
            number_list = get_difference_numbers_list(&numbers_list_maps.get(&(&i - 1)).unwrap());
            numbers_list_maps.insert(i, number_list.clone());
        }
        let mut all_zeros = true;
        for number in number_list {
            if number != 0 {
                all_zeros = false;
                break;
            }
        }
        if all_zeros {
            break;
        }
        i += 1;
    }
    let map_size = numbers_list_maps.keys().count();
    let start_index_backwards = map_size - 1;
    let mut x = start_index_backwards;
    let mut first_number = 0;
    loop {
        let mut number_list = numbers_list_maps.get(&x).unwrap();
        first_number = number_list.get(0).unwrap() - first_number.clone();
        if x == 0 {
            break;
        }
        x -= 1;
    }
    first_number
}

fn get_difference_numbers_list(input_numbers: &Vec<i64>) -> Vec<i64>{
    let mut numbers = Vec::new();
    let mut i = 0;
    for number in input_numbers {
        if i == 0 {
            i += 1;
            continue
        }
        let difference = number - input_numbers.get(i-1).unwrap();
        numbers.push(difference);
        i += 1;
    }
    numbers
}
fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename);
    let lines = io::BufReader::new(file.unwrap()).lines();
    let mut lines_in_string = Vec::new();
    for line in lines {
        lines_in_string.push(line.unwrap());
    }
    lines_in_string
}

fn get_numbers_from_regex(line: &str, re: Regex) -> Vec<i64>{
    let result = re.captures_iter(line);
    let mut numbers = Vec::new();
    for mat in result {
        let number: i64 = mat.iter().next().unwrap().unwrap().as_str().to_string().parse().unwrap();
        numbers.push(number);
    }
    numbers
}