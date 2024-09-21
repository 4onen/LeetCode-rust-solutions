// https://leetcode.com/problems/lexicographical-numbers/

pub struct Solution;

// Initial sol'n (with unnecessary outermost loop removed)
// impl Solution {
//     pub fn lexical_order(n: i32) -> Vec<i32> {
//         fn dfs(n: i32, res: &mut Vec<i32>, cur: i32) {
//             res.push(cur);
//             for i in 0..10 {
//                 if cur * 10 + i > n {
//                     break;
//                 }
//                 dfs(n, res, cur * 10 + i);
//             }
//         }
//         assert!(n >= 1);
//         let mut res = Vec::with_capacity(n as usize);
//         for i in 1..=9.min(n) {
//             dfs(n, &mut res, i);
//         }
//         res
//     }
// }

// Roll-my-own recursion sol'n (broken)
// impl Solution {
//     pub fn lexical_order(n: i32) -> Vec<i32> {
//         assert!(n >= 1);
//         assert!(n <= 50000);
//         let mut res = Vec::with_capacity(n as usize);
//         let mut cur_arr = [0; 5];
//         let mut i_arr = [0; 5];
//         let mut attempts = 40;
//         for i_outermost in 1..=9.min(n) {
//             cur_arr[0] = i_outermost;
//             i_arr[0] = 0;
//             let mut depth = 1;
//             loop {
//                 assert!(attempts > 0);
//                 attempts -= 1;
//                 depth -= 1;
//                 let cur = cur_arr[depth];
//                 let i = i_arr[depth];
//                 dbg!(depth, cur, i);
//                 res.push(cur);
//                 if i > 9 {
//                     if depth <= 0 {
//                         break;
//                     }
//                     continue;
//                 }
//                 let next = cur * 10 + i;
//                 if next > n {
//                     if depth <= 0 {
//                         break;
//                     }
//                     continue;
//                 }
//                 i_arr[depth] = i + 1;
//                 depth += 1;
//                 cur_arr[depth] = next;
//                 i_arr[depth] = 0;
//                 depth += 1;
//             }
//         }
//         res
//     }
// }

// Iterative sol'n (based on best)
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        type NumType = i32;
        assert!(n >= 1);
        assert!(n <= 50000);
        let n = n as NumType;
        let mut res = Vec::with_capacity(n as usize);
        let mut cur = 1 as NumType;
        res.push(cur as i32);
        loop {
            let times_ten = cur.saturating_mul(10);
            if times_ten <= n {
                cur = times_ten;
                res.push(cur as i32);
                continue;
            }
            let plus_one = cur + 1;
            if plus_one <= n && cur % 10 < 9 {
                cur = plus_one;
                res.push(cur as i32);
                continue;
            }
            cur = cur / 10;
            while cur % 10 == 9 {
                cur = cur / 10;
            }
            if cur == 0 {
                break;
            }
            cur += 1;
            res.push(cur as i32);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: i32, expected: &[i32]) {
        assert_eq!(Solution::lexical_order(n), expected);
    }

    fn property_test(n: i32) {
        let res = Solution::lexical_order(n);
        assert_eq!(res.len(), n as usize);
        let mut last = "0".to_string();
        for &x in &res {
            assert!(x > 0);
            assert!(x <= n);
            let s = x.to_string();
            assert!(last < s);
            last = s;
        }
    }

    #[test]
    fn ex1() {
        test(13, &[1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn ex2() {
        test(2, &[1, 2])
    }

    #[test]
    fn myex1() {
        test(1, &[1])
    }

    #[test]
    fn myex10() {
        test(10, &[1, 10, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn myex100() {
        test(
            100,
            &[
                1, 10, 100, 11, 12, 13, 14, 15, 16, 17, 18, 19, 2, 20, 21, 22, 23, 24, 25, 26, 27,
                28, 29, 3, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 4, 40, 41, 42, 43, 44, 45, 46,
                47, 48, 49, 5, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 6, 60, 61, 62, 63, 64, 65,
                66, 67, 68, 69, 7, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 8, 80, 81, 82, 83, 84,
                85, 86, 87, 88, 89, 9, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
            ],
        )
    }

    #[test]
    fn myex50000() {
        property_test(50000)
    }

    #[test]
    fn myex49999() {
        property_test(49999)
    }
}
