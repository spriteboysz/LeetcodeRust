use std::collections::HashMap;

/**
 * Author: Deean
 * Date: 2023-10-22 12:49
 * FileName: lcr0004
 * Description: LCR 004. 只出现一次的数字 II
 */
#[allow(unused)]
pub struct Solution {}

impl Solution {
    #[allow(unused)]
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut hash = HashMap::new();
        for num in nums {
            let mut v = hash.entry(num).or_insert(0);
            *v += 1;
        }
        for (k, v) in hash {
            if v == 1 {
                return k;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self() {
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 100]), 100);
    }
}