use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect("file could not be opened");
    let mut lines = String::new();
    let _ = file.read_to_string(&mut lines);

    let mut sum = 0;
    for line in lines.lines() {
        let digits: Vec<u32> = line
            .chars()
            .filter_map(|character| {
                if character.is_numeric() {
                    Some(character.to_digit(10).unwrap())
                } else {
                    None
                }
            })
            .collect();

        sum += format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<u32>()
            .unwrap();
    }

    println!("{}", sum);
}
