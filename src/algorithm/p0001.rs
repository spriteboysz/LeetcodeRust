/**
 * Author: Deean
 * Date: 2023-10-22 11:23
 * FileName: p0001
 * Description:
 */
#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, x) in nums.iter().enumerate() {
            for (j, y) in nums.iter().enumerate() {
                if j <= i { continue; }
                if x + y == target { return vec![i as i32, j as i32]; }
            }
        }
        return vec![0, 0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}