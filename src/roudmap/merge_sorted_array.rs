// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
//
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//
// The final sorted array should not be returned by the function, but instead be stored inside the array nums1.
// To accommodate this, nums1 has a length of m + n,
// where the first m elements denote the elements that should be merged,
// and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) {
    let (mut m, mut n) = (m as usize, n as usize);

    while n > 0 {
        if m > 0 && nums1[m - 1] > nums2[n - 1] {
            nums1[m + n - 1] = nums1[m - 1];
            m -= 1;
        } else {
            nums1[m + n - 1] = nums2[n - 1];
            n -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::*;

    #[test]
    fn tests_two_sum() {
        let start = Instant::now();
        let mut nums1 = vec!(1, 2, 3, 0, 0, 0);
        merge(&mut nums1, 3, vec!(2, 5, 6), 3);
        assert_eq!(nums1, vec!(1, 2, 2, 3, 5, 6));

        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}
