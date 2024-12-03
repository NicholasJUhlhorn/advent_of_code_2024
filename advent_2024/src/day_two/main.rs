// Nicholas J Uhlhorn
// December 2024
// Day two of Advent of Code

use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day_two.txt")
        .expect("Could not read \"input/day_two.txt\"");

    let mut safe_count = 0;

    for raw_report in contents.split('\n') {
        let report : Vec<i32> = raw_report.split_whitespace()
            .map(|i| i.parse::<i32>().unwrap()).collect();

        let report_length = report.len();
        if report_length <= 1 {
            continue;
        }

        let is_increasing = (report[0] - report[1]).signum() < 0;

        for i in 0..report_length {
            if i == report_length - 1 {
                safe_count += 1;
                break;
            }

            let difference = report[i] - report[i+1];

            if  (difference == 0) || 
                (difference > 0 && is_increasing) ||
                (difference < 0 && !is_increasing) ||
                (difference.abs() > 3) {
                break;
            }
        }
    }  

    println!("Safe report count: {safe_count}");
}

