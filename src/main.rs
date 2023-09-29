use crate::faang::{length_of_longest_substring, longest_palindrome};
use crate::faang::two_sum;

mod faang;

fn main() {
    let two_sum_res = two_sum::run(vec!(2, 7, 11, 15), 9);
    println!("Two sum return: {:?}", two_sum_res);

    let length_of_longest_substring = length_of_longest_substring::run("abcabcbb".to_string());
    println!("Length of longest substring return: {:?}", length_of_longest_substring);

    let longest_palindrome = longest_palindrome::run("abcabcbb".to_string());
    println!("Longest palindrome return: {:?}", longest_palindrome);
}