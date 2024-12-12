// Nicholas J Uhlhorn
// December 2024
// Day two of Advent of Code

use std::fs;
use std::cmp;

fn main() {
    let contents = fs::read_to_string("input/day_two.txt")
        .expect("Could not read \"input/day_two.txt\"");

    let mut strict_safe_count = 0;
    let mut safe_count = 0;

    for raw_report in contents.split('\n') {
        let report : Vec<i32> = raw_report.split_whitespace()
            .map(|i| i.parse::<i32>().unwrap()).collect();

        let report_length = report.len();
        if report_length <= 2 {
            continue;
        }
        
        // for inc/dec we can have at most 2 swaps as in we swapped  (using our mistake allowance)
        // for jumps > |diff| we can either have 0 or 1 (with a mistake) as 1 2 7 3 | 1 7 9 | 8 7 9 6 
        //                                                                  1 5 2     6 2
        //                                                                   6 5       7 
        let mut increasing_levels = 0;
        let mut decreasing_levels = 0;
        let mut invalid_primary_jumps = 0;
        let mut invalid_secondary_jumps = 0;

        for i in 0..report_length-1 {
            let difference = report[i] - report[i+1];
            
            if difference.signum() < 0 {
                increasing_levels += 1;
            } else {
                decreasing_levels += 1;
            }

            if difference == 0 {
                invalid_primary_jumps += 2; // we can't allow other primary cuts so we add 2
            }

            if difference.abs() > 3 {
                invalid_primary_jumps += 1;
                if i < report_length - 2 && (report[i] - report[i+2]).abs() > 3 {
                    invalid_secondary_jumps += 1;
                }
            }


        }

        let min = cmp::min(increasing_levels, decreasing_levels);
        if min > 1 {
            continue;
        }

        if invalid_primary_jumps > 2 || invalid_secondary_jumps > 1 {
            continue;
        }

        if min == 0 && invalid_primary_jumps == 0 {
            strict_safe_count += 1;
        }
        safe_count += 1;

    }  

    println!("Safe report count: {strict_safe_count}\nWith problem dampener: {safe_count}");
}

