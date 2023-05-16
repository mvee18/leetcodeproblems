struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Very naive approach: iterate through the vector and check if the sum of the current value and the next value is equal to the target.
        // If it is, then return the indices of the current value and the next value.
        // If not, then continue iterating.
        let length = nums.len();

        for i in 0..length {
            for j in i + 1..length {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![];
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![2, 5, 5, 11], 10), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![0, 4, 3, 0], 0), vec![0, 3]);
    }
}
