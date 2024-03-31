use std::collections::HashMap;

pub fn hashmap_of_digits() -> HashMap<&'static str, i32> {
    HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ])
}

pub fn create_digit_index_map(line: &str, digit_table: HashMap<&str, i32>) -> HashMap<usize, i32> {
    let mut digit_map = HashMap::new();

        for (digit_str, digit) in digit_table.clone() {
            if line.contains(digit_str) {
                let index: Vec<_> = line
                    .match_indices(digit_str)
                    .map(|(index, _)| (index, digit))
                    .collect();

                digit_map.extend(index);
            };
        }
    
    digit_map
}

pub fn format_digits_to_number(digit_map: HashMap<usize, i32>) -> u32 {
    let mut digit_map_vec: Vec<(usize, i32)> = digit_map.into_iter().collect();
        digit_map_vec.sort();

        let first_digit = digit_map_vec.first().map(|(_, digit)| digit).unwrap();
        let last_digit = digit_map_vec.last().map(|(_, digit)| digit).unwrap();

        let number = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap();

        number
}