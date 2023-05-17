use std::collections::HashSet;
use std::iter::FromIterator;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let length = nums.len();
        // We are going to Hash the values of the vector. If the length of the hashed set is not equal, then we have duplicates.
        let set = HashSet::<i32>::from_iter(nums.into_iter());
        set.len() != length
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_217() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}
