use std::cmp::max;
use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for i in nums.iter() {
            set.insert(*i);
        }

        let mut max_seq = 0;
        for &i in nums.iter() {
            if !set.contains(&(i - 1)) {
                let mut cur = i;
                let mut cur_seq = 1;
                while set.contains(&(cur + 1)) {
                    cur += 1;
                    cur_seq += 1;
                }
                max_seq = max(max_seq, cur_seq);
            }
        }
        max_seq
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_128() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums), 4);
    }

    #[test]
    fn test_128_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 9);
    }
}
