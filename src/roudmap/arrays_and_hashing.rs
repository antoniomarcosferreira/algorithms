use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = HashSet::new();

    for &n in nums.iter() {
        if map.contains(&n) {
            return true;
        }

        map.insert(n);
    };

    false
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_two_sum() {
        let res = contains_duplicate(vec!(1, 2, 3, 4, 5, 1));
        assert_eq!(res, true);

        let res = contains_duplicate(vec!(1, 2, 3, 4, 5));
        assert_eq!(res, false);
    }
}
