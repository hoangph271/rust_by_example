use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;

fn main() {
    /*
    ? Problem #0
    * Given a list of integers,
    * use a vector and return the mean (the average value),
    * median (when sorted, the value in the middle position),
    * and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    */
    // let numbers = vec![0, 2, 4, 4, 6, 1, 3, 2, 0, 2, 0];
    // let mean = numbers.iter().fold(0, |acc, num| acc + num);
    // println!("Mean: {}", mean);
    // let mut sorted_numbers = numbers.clone();
    // sorted_numbers.sort_by(|a, b| if a - b > 0 { Greater } else { Less });
    // println!("Median: {}", sorted_numbers[sorted_numbers.len() / 2]);
    // let mut num_count = HashMap::new();
    // for number in numbers {
    //     let count = num_count.entry(number).or_insert(0);
    //     *count += 1;
    // }
    // let max_count = match num_count.values().max() {
    //     Some(x) => *x,
    //     None => 0,
    // };
    // println!("{}", max_count);
    // let mode_num_counts: Vec<_> = num_count
    //     .iter()
    //     .filter(|(_, val)| **val == max_count)
    //     .collect();
    // let mode_nums: Vec<String> = mode_num_counts
    //     .iter()
    //     .map(|(key, _)| key.to_string())
    //     .collect();
    // println!("Mode: {}...!", mode_nums.join(", "));

    /*
    ? Problem #1
    * Convert strings to pig latin.
    * The first consonant of each word is moved to the end of the word
    * and “ay” is added,so “first” becomes “irst-fay.”
    * Words that start with a vowel have “hay” added to the end instead
    * (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    */
}
