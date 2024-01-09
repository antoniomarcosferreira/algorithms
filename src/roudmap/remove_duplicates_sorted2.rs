// //Given an integer array nums sorted in non-decreasing order,
// // remove some duplicates in-place such that each unique element appears at most twice.
// // The relative order of the elements should be kept the same.
// //
// // Since it is impossible to change the length of the array in some languages,
// // you must instead have the result be placed in the first part of the array nums. More formally,
// // if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result.
// // It does not matter what you leave beyond the first k elements.
// //
// // Return k after placing the final result in the first k slots of nums.
// //
// // Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
// //https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/description/?envType=study-plan-v2&envId=top-interview-150
// pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//     let mut nz = 0;
//     let mut p = 1;
//     let mut pez = 1;
//     for n in nums {
//         p *= *n;
//         if *n == 0 {
//             nz += 1;
//         } else {
//             pez *= *n;
//         }
//     }
//     nums.into_iter().map(|n|
//         match nz {
//             0 => p / n.clone(),
//             1 if n.clone() == 0 => pez,
//             _ => 0
//         }
//     ).collect()
// }
//
// #[cfg(test)]
// mod test {
//     use std::time::Instant;
//
//     use super::*;
//
//     #[test]
//     fn tests_remove_duplicates() {
//         let start = Instant::now();
//
//         let res = remove_duplicates(&mut vec!(1, 1, 2));
//         assert_eq!(res, 2);
//
//         let duration = start.elapsed();
//         println!("Time elapsed in expensive_function() is: {:?}", duration);
//     }
// }
