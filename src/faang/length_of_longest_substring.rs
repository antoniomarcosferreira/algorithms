// 3. Longest Substring Without Repeating Characters
// Medium
// Topics
// Companies
// Given a string s, find the length of the longest
// substring
// without repeating characters.
//
//
//
// Example 1:
//
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:
//
// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:
//
// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//
//
// Constraints:
//
// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.

use std::collections::HashMap;

pub fn run(s: String) -> i32 {
    let mut hash: HashMap<char, i32> = HashMap::new();
    let mut res = 0;
    let mut lo = -1;
    for (hi, ch) in s.chars().enumerate() {
        if let Some(i) = hash.insert(ch, hi as i32) {
            lo = lo.max(i);
        }
        res = res.max(hi as i32 - lo);
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_two_sum() {
        let mut res = run("abcabcbb".to_string());
        assert_eq!(res, 3);

        res = run("bbbbb".to_string());
        assert_eq!(res, 1);

        res = run("pwwkew".to_string());
        assert_eq!(res, 3);
    }
}