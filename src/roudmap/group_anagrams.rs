use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut key = [0_u8; 26];

        for c in s.chars() {
            key[c as usize - 'a' as usize] += 1;
        }

        if let Some(vals) = map.get_mut(&key) {
            vals.push(s);
        } else {
            map.insert(key, vec![s]);
        }
    }

    map.into_values().collect()
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::*;

    #[test]
    fn tests_two_sum() {
        let start = Instant::now();

        let data = vec!["ate".to_string(), "casa".to_string(), "eat".to_string(), "aet".to_string(), "hum".to_string(), "saca".to_string()];
        let res = group_anagrams(data);


        let expected = vec![vec!["ate".to_string(), "eat".to_string(), "aet".to_string()], vec!["hum".to_string()], vec!["casa".to_string(), "saca".to_string()]];
        for i in res.iter() {
            assert!(expected.contains(i));
        }
 
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}
