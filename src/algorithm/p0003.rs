/**
 * Author: Deean
 * Date: 2022-10-09 23:09
 * FileName: algorithm/P0001.rs
 * Description: 1. 两数之和
 */

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let seq: Vec<char> = s.chars().collect();
        let len = seq.len();
        let (mut start, mut end, mut max) = (0, 0, 0);

        while end < len {
            for idx in start..end {
                if seq[end] == seq[idx] {
                    start = idx + 1;
                    break;
                }
            }
            let curr = end - start + 1;
            if curr > max {
                max = curr
            }
            end += 1
        }
        max as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let abc = Solution::length_of_longest_substring("abcabcbb".to_string());
        println!(abc);
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}