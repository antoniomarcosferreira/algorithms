pub fn valid_anagram(s: String, t: String) -> bool {
    if t.len() != s.len() {
        return false;
    }

    let mut s1: Vec<char> = s.chars().collect();
    s1.sort_unstable();
    let mut t1: Vec<char> = t.chars().collect();
    t1.sort_unstable();
    s1.into_iter().collect::<String>() == t1.into_iter().collect::<String>()

    // if t.len() != s.len() {
    //     return false;
    // }
    //
    // let mut map: HashMap<char, i64> = HashMap::new();
    //
    // for (a, b) in s.chars().zip(t.chars()) {
    //     *map.entry(a).or_default() += 1;
    //     *map.entry(b).or_default() -= 1;
    // }
    //
    // map.into_values().all(|cnt| cnt == 0)
}


#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::*;

    #[test]
    fn tests_two_sum() {
        let start = Instant::now();


        let res = valid_anagram("marcos".to_string(), "ramsoc".to_string());
        assert_eq!(res, true);

        let res = valid_anagram("marcosmarcosmarcosmarcosmarcosmarcosmarcosmarcosmarcosmarcosmarcos".to_string(), "ramsocramsocramsocramsocramsocramsocramsocramsocramsocramsocramsoc".to_string());
        assert_eq!(res, true);


        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}
