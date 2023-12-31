/*1. Two Sum
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order.

Example 1:
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

Example 2:
Input: nums = [3,2,4], target = 6
Output: [1,2]

Example 3:
Input: nums = [3,3], target = 6
Output: [0,1]

Constraints:
2 <= nums.length <= 104
-109 <= nums[i] <= 109
-109 <= target <= 109
Only one valid answer exists.
*/


use std::collections::HashMap;

pub fn run(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (i, n) in nums.iter().enumerate() {
        if let Some(&ci) = map.get(&(target - n)) { return vec![ci as i32, i as i32]; }
        map.insert(n, i);
    }
    return vec![];
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_two_sum() {
        let mut res = run(vec!(2, 7, 11, 15), 9);
        assert_eq!(res, vec!(0, 1));

        res = run(vec!(3, 2, 4), 6);
        assert_eq!(res, vec!(1, 2));

        res = run(vec!(3, 3), 6);
        assert_eq!(res, vec!(0, 1));
    }
}
