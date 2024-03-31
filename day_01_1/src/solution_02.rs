use std::{fs::File, io::Read, path::Path};

// with only iterators, no for loops are used
pub fn solution_02() {
    let path = Path::new("input.txt");
    let mut file = File::open(path).unwrap();
    let mut lines = String::new();
    let _ = file.read_to_string(&mut lines);

    let sum: u32 = lines
        .lines()
        .filter_map(|line| {
            let digits: Vec<u32> = get_digits_from_line(line);
            let number = number_from_digits(digits);
            Some(number)
        })
        .sum();

    println!("{}", sum);
}

fn get_digits_from_line(line: &str) -> Vec<u32> {
    line
        .chars()
        .filter_map(|character| {
            if character.is_numeric() {
                let digit = character.to_digit(10).unwrap();
                Some(digit)
            } else {
                None
            }
        })
        .collect()
}

fn number_from_digits(digits: Vec<u32>) -> u32 {
    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse::<u32>()
        .unwrap()
}