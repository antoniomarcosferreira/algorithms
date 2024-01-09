// you can use includes, for example:
// use std::cmp;

use std::collections::HashMap;

pub fn solution(A: &mut Vec<i32>) -> i32 {
    // Implement your solution here
    let mut positive_vals = Vec::new();

    for n in A.iter() {
        if n > &0 {
            positive_vals.push(n.clone())
        }
    }

    for i in 1..=(A.len() + 1) {
        let key = i as i32;
        if !positive_vals.contains(&key) {
            return key;
        }
    }
    1
}
 
pub fn solutionB(B: &mut Vec<i32>) -> i32 {
    // Implement your solution here
    let mut positive_vals = HashMap::new();

    for n in B.iter() {
        if n > &0 {
            positive_vals.insert(n, true);
        }
    }

    for i in 1..=(B.len() + 1) {
        let key = i as i32;
        if !positive_vals.contains_key(&key) {
            return key;
        }
    }
    1
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_test() {
        let mut res = solution(&mut vec!(1, -1, 3, 4, 2, 1));
        assert_eq!(res, 5);

        res = solution(&mut vec!(-1, -1));
        assert_eq!(res, 1);
    }

    #[test]
    fn solutionB_test() {
        let mut res = solutionB(&mut vec!(1, -1, 3, 4, 2, 1));
        assert_eq!(res, 5);

        res = solutionB(&mut vec!(-1, -1));
        assert_eq!(res, 1);
    }
}
