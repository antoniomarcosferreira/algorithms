// 238. Product of Array Except Self
// Solved
// Medium
// Topics
// Companies
// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
//
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
//
// You must write an algorithm that runs in O(n) time and without using the division operation.
//


pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut nz = 0;
    let mut p = 1;
    let mut pez = 1;
    for n in &nums {
        p *= *n;
        if *n == 0 {
            nz += 1;
        } else {
            pez *= *n;
        }
    }
    nums.into_iter().map(|n|
        match nz {
            0 => p / n,
            1 if n == 0 => pez,
            _ => 0
        }
    ).collect()
}


#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::*;

    #[test]
    fn tests_product_except_self() {
        let start = Instant::now();

        let mut res = product_except_self(vec!(1, 2, 3, 4));
        assert_eq!(res, vec!(24, 12, 8, 6));

        res = product_except_self(vec!(-1, 1, 0, -3, 3));
        assert_eq!(res, vec!(0, 0, 9, 0, 0));

        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}