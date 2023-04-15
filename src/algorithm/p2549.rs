/**
 * Author: Deean
 * Date: 2023-04-15 23:05
 * FileName: src/algorithm
 * Description: 
 */
#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn distinct_integers(n: i32) -> i32 {
        if n == 1 { 1 } else { n - 1 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self() {
        assert_eq!(Solution::distinct_integers(5), 4);
    }
}