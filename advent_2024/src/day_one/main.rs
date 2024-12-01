// Nicholas J Uhlhorn
// December 2024
// Day One of Advent of code

// use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day_one.txt")
        .expect("Could not read \"input.txt\"");

    let mut list_1 = vec!(0; 0);
    let mut list_2 = vec!(0; 0);

    for (i, token) in contents.split_whitespace().enumerate() {
        if i % 2 == 0 {
            list_1.push(token.parse::<i32>().expect("Malformed string token -> int: {token}"))
        } else {
            list_2.push(token.parse::<i32>().expect("Malformed string token -> int: {token}"))
        }
    }

    assert!(list_1.len() == list_2.len(), "The length of list_1 and list_2 do not match!");

    list_1.sort();
    list_2.sort();

    let mut sum = 0;
    for i in 0..list_1.len() {
        let difference = (list_1[i] - list_2[i]).abs();
        sum = sum + difference;
    }

    println!("The difference sum is: {sum}")
}