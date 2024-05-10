// https://leetcode.com/problems/k-th-smallest-prime-fraction/

pub struct Solution;

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, mut k: i32) -> Vec<i32> {
        #[derive(Eq)]
        struct Rational {
            numerator: u16,
            denominator: u16,
        }
        impl std::cmp::PartialEq for Rational {
            fn eq(&self, other: &Self) -> bool {
                self.numerator as u32 * other.denominator as u32
                    == self.denominator as u32 * other.numerator as u32
            }
        }
        impl std::cmp::PartialOrd for Rational {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(
                    (self.numerator as u32 * other.denominator as u32)
                        .cmp(&(self.denominator as u32 * other.numerator as u32)),
                )
            }
        }
        impl std::cmp::Ord for Rational {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                (self.numerator as u32 * other.denominator as u32)
                    .cmp(&(self.denominator as u32 * other.numerator as u32))
            }
        }
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..arr.len() as u16 {
            for j in i + 1..arr.len() as u16 {
                heap.push(std::cmp::Reverse(Rational {
                    numerator: arr[i as usize] as u16,
                    denominator: arr[j as usize] as u16,
                }));
            }
        }
        loop {
            k -= 1;
            if k <= 0 {
                break;
            }
            heap.pop();
        }
        let r = heap.pop().unwrap().0;
        vec![r.numerator as i32, r.denominator as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_prime(num: u16) -> bool {
        match num {
            0 => false,
            1 => true, // For our purposes, easier to just do this.
            2 => true,
            _ => {
                let mut i = 2;
                while i * i <= num {
                    if num % i == 0 {
                        return false;
                    }
                    i += 1;
                }
                return true;
            }
        }
    }

    fn test(arr: &[u16], k: u32, expected: &[u16]) {
        assert!(arr.len() >= 2);
        assert!(arr.len() <= 1000);
        assert_eq!(arr[0], 1);
        for &x in arr {
            assert!(x >= 1, "x={}", x);
            assert!(x <= 30_000, "x={}", x);
            assert!(is_prime(x), "x={}", x);
        }
        assert!(k > 0);
        assert!(k <= arr.len() as u32 * (arr.len() as u32 - 1) / 2);
        let arr_in: Vec<i32> = arr.into_iter().map(|&x| x as i32).collect();
        let expected_out: Vec<i32> = expected.into_iter().map(|&x| x as i32).collect();
        assert_eq!(
            Solution::kth_smallest_prime_fraction(arr_in, k as i32),
            expected_out,
            "arr={:?}, k={}",
            arr,
            k
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3, 5], 3, &[2, 5]);
    }

    #[test]
    fn ex2() {
        test(&[1, 7], 1, &[1, 7]);
    }

    #[test]
    fn myex1() {
        test(&[1, 2, 3, 5], 1, &[1, 5]);
    }

    #[test]
    fn myex2() {
        test(&[1, 2, 3, 5], 2, &[1, 3]);
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 13, 17, 59], 6, &[13, 17]);
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 7, 23, 29, 47], 8, &[23, 47]);
    }

    #[test]
    fn discussion_case3() {
        test(&[1, 2, 11, 37, 83, 89], 11, &[11, 37]);
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                1, 3, 67, 97, 173, 263, 307, 383, 479, 797, 823, 827, 853, 1039, 1063, 1291, 1543,
                1669, 1901, 1987, 2017, 2029, 2063, 2089, 2351, 2381, 2459, 2531, 2539, 2647, 2801,
                2851, 3067, 3221, 3257, 3499, 3761, 3833, 3853, 3863, 3923, 4021, 4157, 4231, 4349,
                4451, 4567, 4583, 4729, 4969, 5077, 5147, 5441, 5591, 5827, 5897, 6043, 6079, 6257,
                6301, 6361, 6473, 6653, 6737, 6791, 6803, 6949, 7369, 7481, 7499, 7643, 7829, 7949,
                8311, 8539, 8831, 8969, 9181, 9337, 9749, 9851, 9887, 10069, 10141, 10271, 10781,
                10831, 11059, 11113, 11239, 11279, 11411, 11489, 11549, 11621, 11953, 12041, 12377,
                12421, 12497, 12763, 12823, 12953, 12973, 13147, 13679, 13997, 14011, 14029, 14051,
                14489, 14593, 14621, 15199, 15391, 15559, 15607, 15641, 15649, 15749, 16067, 16073,
                16831, 17327, 17449, 17627, 17921, 18047, 18149, 18257, 18289, 18367, 18539, 18743,
                18839, 19433, 19483, 19541, 19813, 19861, 20047, 20233, 20533, 20753, 21179, 21401,
                21517, 21611, 21863, 22039, 22157, 22727, 22739, 22861, 23021, 23159, 23173, 23297,
                23447, 23669, 23671, 23801, 23831, 23857, 23893, 23981, 23993, 24071, 24247, 24533,
                24547, 24709, 25031, 25033, 25117, 25261, 25321, 25453, 25469, 25903, 26227, 26297,
                26713, 27277, 27431, 27941, 28351, 28687, 28697, 28859, 28901, 28933, 29129, 29147,
                29207, 29209, 29401, 29587, 29741, 29881,
            ],
            10453,
            &[12953, 29881],
        );
    }
}
