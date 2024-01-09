// you can use includes, for example:
// use std::cmp;

pub fn solution(s: String) -> bool {
    let b = "b".chars().next().unwrap();

    if s.len() == 1 { return true; }

    let mut index_of_b = s.chars().collect::<Vec<char>>();
    index_of_b.dedup();

    match index_of_b.iter().position(|&r| r == b) {
        Some(i) => if i < 1 { return false; },
        _ => {}
    }
    true
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_test() {
        let mut res = solution("ba".to_string());
        assert_eq!(res, false);

        res = solution("aaaaaabbbbbbbabbbbaaaaa".to_string());
        assert_eq!(res, true);

        res = solution("bbbbaaaabbabbbbb".to_string());
        assert_eq!(res, false);

        res = solution("a".to_string());
        assert_eq!(res, true);

        res = solution("b".to_string());
        assert_eq!(res, true);

        res = solution("".to_string());
        assert_eq!(res, true);
    }
}
