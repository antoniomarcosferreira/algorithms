// 5. Longest Palindromic Substring
// Medium
// Topics
// Companies
// Hint
// Given a string s, return the longest
// palindromic
//
// substring
// in s.
//
//
//
// Example 1:
//
// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.
// Example 2:
//
// Input: s = "cbbd"
// Output: "bb"
//
//
// Constraints:
//
// 1 <= s.length <= 1000
// s consist of only digits and English letters.



pub fn run(s: String) -> String {
    if s.len() == 0 { return "".to_string(); }
    let s: Vec<char> = s.chars().collect();
    let len = s.len();
    let mut start = 0;
    let mut end = 0;

    for i in 0..len {
        let mut left = i;
        let mut right = i;

        while right + 1 < len && s[right + 1] == s[left] {
            right += 1;
        }

        while right + 1 < len && left > 0 && s[right + 1] == s[left - 1] {
            right += 1;
            left -= 1;
        }
        if right - left > end - start {
            start = left;
            end = right;
        }
    }
    s[start..=end].iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_two_sum() {
        let mut res = run("abcda".to_string());
        assert_eq!(res, "a".to_string());

        res = run("babad".to_string());
        assert_eq!(res, "bab".to_string());

        res = run("cbbd".to_string());
        assert_eq!(res, "bb".to_string());

        res = run("".to_string());
        assert_eq!(res, "".to_string());

        res = run("a".to_string());
        assert_eq!(res, "a".to_string());

        res = run("bb".to_string());
        assert_eq!(res, "bb".to_string());

        let res = run("ac".to_string());
        assert_eq!(res, "a".to_string());

        let res = run("ccc".to_string());
        assert_eq!(res, "ccc".to_string());
    }
}
