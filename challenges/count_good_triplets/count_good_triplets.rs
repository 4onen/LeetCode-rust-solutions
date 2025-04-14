// https://leetcode.com/problems/count-good-triplets/

pub struct Solution;

// Naive sol'n walking all triplets
impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let a = a as u32;
        let b = b as u32;
        let c = c as u32;
        let mut count = 0;
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                if i32::abs_diff(arr[i], arr[j]) <= a {
                    for k in j + 1..arr.len() {
                        if i32::abs_diff(arr[j], arr[k]) <= b && i32::abs_diff(arr[i], arr[k]) <= c
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr: &[u16], a: u16, b: u16, c: u16, expected: i32) {
        assert!(arr.len() >= 3);
        assert!(arr.len() <= 100);
        for &num in arr {
            assert!(num >= 0);
            assert!(num <= 1000);
        }
        assert!(a >= 0);
        assert!(a <= 1000);
        assert!(b >= 0);
        assert!(b <= 1000);
        assert!(c >= 0);
        assert!(c <= 1000);
        assert_eq!(
            Solution::count_good_triplets(
                arr.iter().map(|&x| x as i32).collect(),
                a as i32,
                b as i32,
                c as i32
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[3, 0, 1, 1, 9, 7], 7, 2, 3, 4)
    }

    #[test]
    fn ex2() {
        test(&[1, 1, 2, 2, 3, 3], 0, 0, 1, 0)
    }
}
