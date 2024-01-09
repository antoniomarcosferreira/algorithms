use crate::faang::{length_of_longest_substring, longest_palindrome, max_area};
use crate::faang::two_sum;
use crate::roudmap::{group_anagrams, merge_sorted_array, remove_element, valid_anagram};
use crate::roudmap::arrays_and_hashing::contains_duplicate;

mod faang;
mod roudmap;
mod codility;
mod leetcode;

fn main() {
    let two_sum_res = two_sum::run(vec!(2, 7, 11, 15), 9);
    println!("Two sum return: {:?}", two_sum_res);

    let length_of_longest_substring = length_of_longest_substring::run("abcabcbb".to_string());
    println!("Length of longest substring return: {:?}", length_of_longest_substring);

    let longest_palindrome = longest_palindrome::run("abcabcbb".to_string());
    println!("Longest palindrome return: {:?}", longest_palindrome);

    let max_area = max_area::run(vec!(1, 8, 6, 2, 5, 4, 8, 3, 7));
    println!("Max area return: {:?}", max_area);

    // teste the roudmap
    {
        let ret = contains_duplicate(vec![1, 2, 3, 4, 1]);
        println!("Roudmap test: Contains Duplicate: {}", ret);

        // two Sum
        let res = roudmap::two_sum::two_sum(vec!(2, 7, 11, 15), 9);
        println!("Two Sum: {:?}", res);

        //group_anagrams
        let data = vec!["ate".to_string(), "casa".to_string(), "eat".to_string(), "aet".to_string(), "hum".to_string(), "saca".to_string()];
        let res = group_anagrams::group_anagrams(data);
        println!("Group anagrams: {:?}", res);

        //valid_anagram
        let res = valid_anagram::valid_anagram("marcos".to_string(), "ramsoc".to_string());
        println!("Valid_anagram: {:?}", res);


        // Merge sorted array
        let mut nums1 = vec!(1, 2, 3, 0, 0, 0);
        merge_sorted_array::merge(&mut nums1, 3, vec!(2, 5, 6), 3);
        println!("Merge sorted array: {:?}", nums1);

        // remove_element
        let res = remove_element::remove_element(&mut vec!(3, 2, 2, 3), 3);
        println!("Remove_element: {:?}", res);

//remove_duplicates
//         let res = remove_duplicates_sorted::remove_duplicates(&mut vec!(1, 1, 2));
//         println!("Remove duplicates sorted: {:?}", res);
    }
}