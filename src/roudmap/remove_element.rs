// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
// The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
//
// Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:
//
// Change the array nums such that the first k elements of nums contain the elements which are not equal to val.
// The remaining elements of nums are not important as well as the size of nums.
// Return k.

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut matched = 0;
    for cur in 0..nums.len() {
        match nums[cur] == val {
            true => (),
            false => {
                nums[matched] = nums[cur];
                matched += 1;
            }
        };
    }
    return matched as i32;
}


#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::*;

    #[test]
    fn tests_remove_element() {
        let start = Instant::now();
        let res = remove_element(&mut vec!(3, 2, 2, 3), 3);
        assert_eq!(res, 2);

        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}
