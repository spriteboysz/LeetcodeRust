/**
 * Author: Deean
 * Date: 2023-04-15 23:18
 * FileName: src/algorithm
 * Description: 
 */
#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n % 2 == 0 { n } else { n * 2 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self() {
        assert_eq!(Solution::smallest_even_multiple(5), 10);
    }
}