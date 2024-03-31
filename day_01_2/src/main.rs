use std::{fs::File, io::Read, path::Path};

use day_01_2::{create_digit_index_map, format_digits_to_number, hashmap_of_digits};

fn main() {
    let path = Path::new("input.txt");
    let mut file = File::open(path).expect("file could not be opened");
    let mut lines = String::new();
    file.read_to_string(&mut lines)
        .expect("failed to get data from file");

    let digit_table = hashmap_of_digits();

    let sum: u32 = lines
        .lines()
        .map(|line| {
            let digit_map = create_digit_index_map(line, digit_table.clone());
            format_digits_to_number(digit_map)
        })
        .sum();

    println!("{}", sum);
}
