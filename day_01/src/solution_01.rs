use std::{fs::File, io::Read, path::Path};

// for loop is used
pub fn solution_01() {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect("file could not be opened");
    let mut lines = String::new();
    let _ = file.read_to_string(&mut lines);

    let mut sum = 0;
    for line in lines.lines() {
        let digits: Vec<u32> = get_digits_from_line(line);
        sum += number_from_digits(digits);
    }

    println!("{}", sum);
}

fn number_from_digits(digits: Vec<u32>) -> u32 {
    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse::<u32>()
        .unwrap()
}

fn get_digits_from_line(line: &str) -> Vec<u32> {
    line
        .chars()
        .filter_map(|character| {
            if character.is_numeric() {
                Some(character.to_digit(10).unwrap())
            } else {
                None
            }
        })
        .collect()
}