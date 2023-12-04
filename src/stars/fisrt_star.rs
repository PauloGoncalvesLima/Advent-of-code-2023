
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


pub fn get_first_star_value() -> u32{
    let mut numbers: Vec<u32> = Vec::new();
    let mut total:u32 = 0;
    let lines = read_lines("./src/inputs/input-1star.txt");

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        for charecter in line.chars(){
            if let Some(digit) = charecter.to_digit(10) {
                numbers.push(digit);
            }
        }
        if numbers.len() > 0 {
            total += (10 * numbers[0]) + numbers[numbers.len() - 1];
        }
        numbers.clear();
    }

    return total;
}


