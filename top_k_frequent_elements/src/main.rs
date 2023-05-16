use std::collections::HashMap;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Hashmap of number and its frequency
        let mut hash = HashMap::new();

        for i in 0..nums.len() {
            hash.entry(nums[i]).and_modify(|e| *e += 1).or_insert(1);
        }

        // Sort the hashmap by value
        let mut sorted_hash: Vec<(&i32, &i32)> = Vec::from_iter(hash.iter());
        sorted_hash.sort_by(|a, b| b.1.cmp(a.1));

        // Take k elements from the sorted hashmap
        let mut result: Vec<i32> = Vec::new();
        for i in 0..k as usize {
            result.push(*sorted_hash[i].0);
        }

        result
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_k_frequent() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 2, 1, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
