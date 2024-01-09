use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (i, n) in nums.iter().enumerate() {
        if let Some(&ci) = map.get(&(target - n)) { return vec![ci as i32, i as i32]; }
        map.insert(n, i);
    }
    return vec![];
}


#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::*;

    #[test]
    fn tests_two_sum() {
        let start = Instant::now();


        let mut res = two_sum(vec!(2, 7, 11, 15), 9);
        assert_eq!(res, vec!(0, 1));

        res = two_sum(vec!(3, 2, 4), 6);
        assert_eq!(res, vec!(1, 2));

        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}
