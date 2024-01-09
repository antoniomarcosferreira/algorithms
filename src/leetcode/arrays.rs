use std::collections::HashMap;

// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/727/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}


// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/564/
// Best Time to Buy and Sell Stock II
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() { return 0; }

    let mut sum = 0;
    for i in 0..prices.len() - 1 {
        if prices[i + 1] > prices[i] {
            sum += prices[i + 1] - prices[i];
        }
    }
    sum
}


// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/646/
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    // for i in 0..k {
    //     let last = nums.pop().unwrap();
    //     nums.insert(0, last);
    // }
    let i = nums.len() - (k as usize % nums.len());
    nums.append(&mut nums[..i].to_vec());
    let (_, right) = nums.split_at(i);
    *nums = right.to_vec();
}


// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/578/
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut n = nums.clone();
    n.sort_unstable();
    n.dedup();
    (n.len() != nums.len())
    // let mut tmp = HashSet::with_capacity(nums.len());
    // !nums.into_iter().all(|i| tmp.insert(i))
}

// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/549/
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::with_capacity(nums.len());

    for n in nums.iter() {
        match map.get(n) {
            Some(v) => map.insert(n, (v + 1) as i32),
            None => map.insert(n, 1i32),
        };
    }

    for (i, v) in map.iter() {
        if v < &2 {
            return **i;
        }
    }
    0
}

// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/674/

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut pre_target = nums1.clone();
    let mut target = nums2.clone();

    if nums1.len() > nums2.len() {
        pre_target = nums2.clone();
        target = nums1.clone();
    }

    let mut count_map = HashMap::new();

    for num in pre_target.iter() {
        if count_map.contains_key(num) {
            count_map.insert(num, count_map.get(num).unwrap() + 1);
        } else {
            count_map.insert(num, 1);
        }
    }

    let mut result = Vec::new();

    for num in target.iter() {
        if count_map.contains_key(num) {
            result.push(num.clone());

            let count = count_map.get(num).unwrap().clone();

            if count == 1 {
                count_map.remove(num);
            } else {
                count_map.insert(num, count - 1);
            }
        }
    }

    result
}


pub fn intersect2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let (smaller, bigger) = if nums1.len() <= nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };
    let mut map = std::collections::HashMap::<i32, u16>::with_capacity(smaller.len());
    let mut result: Vec<i32> = vec![];

    bigger.iter().for_each(|&n| {
        map.entry(n).and_modify(|v| *v += 1).or_insert(1);
    });

    for &n in smaller.iter() {
        if let Some(&v) = map.get(&n) {
            if v > 0 {
                result.push(n);
                map.entry(n).and_modify(|v| *v -= 1);
            }
        }
    }

    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn remove_duplicates_test() {
        let mut res = remove_duplicates(&mut vec!(0, 0, 1, 1, 1, 2, 2, 3, 3, 4));
        assert_eq!(res, 5);

        res = remove_duplicates(&mut vec!(1, 1, 2));
        assert_eq!(res, 2);
    }


    #[test]
    fn max_profit_test() {
        let mut res = max_profit(vec!(7, 1, 5, 3, 6, 4));
        assert_eq!(res, 7);

        res = max_profit(vec!(1, 2, 3, 4, 5));
        assert_eq!(res, 4);


        res = max_profit(vec!(7, 6, 4, 3, 1));
        assert_eq!(res, 0);
    }


    #[test]
    fn rotate_test() {
        let mut arr = vec!(1, 2, 3, 4, 5, 6, 7);
        rotate(&mut arr, 3);
        assert_eq!(arr, vec!(5, 6, 7, 1, 2, 3, 4));
    }


    #[test]
    fn contains_duplicate_test() {
        let mut arr = contains_duplicate(vec!(1, 1, 1, 3, 3, 4, 3, 2, 4, 2));
        assert_eq!(arr, true);
    }

    #[test]
    fn single_number_test() {
        let mut arr = single_number(vec!(1, 1, 1, 3, 3, 4, 3, 2, 4, 2, 9));
        assert_eq!(arr, 9);
    }


    #[test]
    fn intersect_test() {
        let mut arr = intersect(vec!(4, 9, 5), vec!(9, 4, 9, 8, 4));
        assert_eq!(arr, vec!(9, 4));

        arr = intersect(vec!(0, 4, 5), vec!(8, 6, 0, 4, 7, 2));
        assert_eq!(arr, vec!(0, 4));

        arr = intersect2(vec!(4, 9, 5), vec!(9, 4, 9, 8, 4));
        assert_eq!(arr, vec!(9, 4));

        arr = intersect2(vec!(0, 4, 5), vec!(8, 6, 0, 4, 7, 2));
        assert_eq!(arr, vec!(0, 4));
    }
}
