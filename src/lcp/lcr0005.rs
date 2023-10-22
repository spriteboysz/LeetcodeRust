/**
 * Author: Deean
 * Date:  18:48
 * FileName: lcr0005
 * Description:LCR 005. 最大单词长度乘积
 */
#[allow(unused)]
pub struct Solution {}

impl Solution {
    #[allow(unused)]
    pub fn max_product(words: Vec<&str>) -> i32 {
        let n = words.len();
        let mut hashes = vec![0; n];

        for i in 0..n {
            let word = words[i].as_bytes();
            let m = word.len();

            for j in 0..m {
                hashes[i] = hashes[i] | (1 << (word[j] as usize - b'a' as usize));
            }
        }

        let mut maximum = 0;
        for i in 0..n {
            for j in i + 1..n {
                if hashes[i] & hashes[j] == 0 {
                    maximum = maximum.max(words[i].len() * words[j].len());
                }
            }
        }
        maximum as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self() {
        assert_eq!(Solution::max_product(vec!["abcw", "baz", "foo", "bar", "fxyz", "abcdef"]), 16);
    }
}