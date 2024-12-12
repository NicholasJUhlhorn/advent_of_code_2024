// Nicholas J Uhlhorn
// December 2024

use std::fs;

fn get_raw_mul_operations(input: String) -> Vec<(i32, i32)>{
    let raw_tokens: Vec<&str> = input.split("mul(").collect();
    let mut checked_tokens: Vec<(i32, i32)> = vec![];

    for token in raw_tokens {
        let valid_close_string = match token.split_once(")"){
            Some(x) => x.0,
            None => continue,
        };

        let parts = valid_close_string.split_once(',');
        let unchecked = match parts {
            Some(x) => (x.0, x.1),
            None => continue,
        };

        let part_one = match unchecked.0.parse::<i32>() {
            Ok(x) => x,
            Err(_) => continue,
        };    

        let part_two = match unchecked.1.parse::<i32>() {
            Ok(x) => x,
            Err(_) => continue,
        };    

        checked_tokens.push((part_one, part_two))
    }

    return checked_tokens;
}

fn main() {
    let contents = fs::read_to_string("input/day_three.txt")
        .expect("Could not read \"input.txt\"");

    let mul_inputs = get_raw_mul_operations(contents);

    let mut sum = 0;
    for input in mul_inputs {
        sum += input.0 * input.1;
    }

    println!("Sum of all mul operands: {}", sum);
}

