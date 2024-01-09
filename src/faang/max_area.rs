/*
11. Container With Most Water
Medium
Topics
Companies
Hint
You are given an integer array height of length n.
There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
Find two lines that together with the x-axis form a container, such that the container contains the most water.
Return the maximum amount of water a container can store.
Notice that you may not slant the container.


Example 1:
Input: height = [1,8,6,2,5,4,8,3,7]
Output: 49
Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7].
In this case, the max area of water (blue section) the container can contain is 49.

Example 2:
Input: height = [1,1]
Output: 1
Constraints:
n == height.length
2 <= n <= 105
0 <= height[i] <= 104
*/


pub fn run(height: Vec<i32>) -> i32 {
    let (mut left, mut right, mut content) = (0, (height.len() - 1) as usize, 0);
    while right != left {
        content = content.max(height[left].min(height[right]) as usize * (right - left));
        if height[left] < height[right] { left += 1; } else { right -= 1; }
    }
    content as i32
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_two_sum() {
        let mut res = run(vec!(1, 8, 6, 2, 5, 4, 8, 3, 7));
        assert_eq!(res, 49);

        res = run(vec!(1, 1));
        assert_eq!(res, 1);
    }
}
