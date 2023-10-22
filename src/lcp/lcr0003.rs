/**
 * Author: Deean
 * Date: 2023-10-22 11:42
 * FileName: lcr0003
 * Description: LCR 003. 比特位计数
 */
#[allow(unused)]
pub struct Solution {}

impl Solution {
    #[allow(unused)]
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut bits = vec![];
        for i in 0..=n {
            let mut num = i;
            let mut cnt = 0;
            while num > 0 {
                num = num & num - 1;
                cnt += 1;
            }
            bits.push(cnt);
        }
        return bits;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}