// https://leetcode.com/problems/maximize-happiness-of-selected-children/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
//         assert!(k > 0);
//         let n = happiness.len();
//         assert!(n >= k as usize);
//         happiness.select_nth_unstable(n - k as usize);
//         let selected = &mut happiness[n - k as usize..];
//         selected.sort_unstable();
//         let mut result = 0;
//         let mut happiness_lost = 0;
//         for &mut child in selected.into_iter().rev() {
//             if happiness_lost >= child {
//                 break;
//             }
//             result += (child - happiness_lost) as i64;
//             happiness_lost += 1;
//         }
//         result
//     }
// }

// My fastest sol'n
// impl Solution {
//     pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
//         assert!(k > 0);
//         let n = happiness.len();
//         assert!(n >= k as usize);
//         happiness.select_nth_unstable_by_key((k - 1) as usize, |&x| std::cmp::Reverse(x));
//         let selected = &mut happiness[..k as usize];
//         selected.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
//         selected
//             .into_iter()
//             .enumerate()
//             .map(|(happiness_lost, &mut child)| {
//                 if happiness_lost as i32 > child {
//                     0
//                 } else {
//                     (child - happiness_lost as i32) as i64
//                 }
//             })
//             .sum()
//     }
// }

// Further optimized sol'n (yet slower)
// impl Solution {
//     pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
//         assert!(k > 0);
//         let k = k as usize;
//         let n = happiness.len();
//         assert!(n >= k);
//         let selected = if k == n {
//             &mut happiness
//         } else {
//             happiness.select_nth_unstable_by_key(k - 1, |&x| std::cmp::Reverse(x));
//             &mut happiness[..k]
//         };
//         selected.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
//         let mut result = 0;
//         for (happiness_lost, child) in selected
//             .iter()
//             .enumerate()
//             .map(|(idx, &child)| (idx as i32, child))
//         {
//             if happiness_lost >= child {
//                 break;
//             }
//             result += (child - happiness_lost) as i64;
//         }
//         result
//     }
// }

// Further optimized sol'n (yet slower)
// impl Solution {
//     pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
//         assert!(k > 0);
//         let n = happiness.len();
//         assert!(n >= k as usize);
//         happiness.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
//         let mut result = 0;
//         for i in 0..k as i32 {
//             let child = happiness[i as usize];
//             if i as i32 >= child {
//                 break;
//             }
//             result += (child - i as i32) as i64;
//         }
//         result
//     }
// }

// Further optimized but without type conversions (Finally 19ms again)
impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        assert!(k > 0);
        let n = happiness.len();
        assert!(n >= k as usize);
        happiness.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
        let mut result = 0;
        for i in 0..k as i64 {
            let child = happiness[i as usize] as i64;
            if i >= child {
                break;
            }
            result += child - i;
        }
        result
    }
}

// Try -x instead of std::cmp::Reverse(x) (Worst sol'n yet - 25ms)
// impl Solution {
//     pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
//         assert!(k > 0);
//         let n = happiness.len();
//         assert!(n >= k as usize);
//         happiness.sort_unstable_by_key(|&x| -x);
//         let mut result = 0;
//         for i in 0..k as i64 {
//             let child = happiness[i as usize] as i64;
//             if i >= child {
//                 break;
//             }
//             result += child - i;
//         }
//         result
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(happiness: &[u32], k: u32, expected: u64) {
        assert!(happiness.len() >= k as usize);
        assert!(happiness.len() < 2_000_000);
        let happiness: Vec<i32> = happiness
            .iter()
            .map(|&x| {
                assert!(x > 0);
                assert!(x < 100_000_000);
                x as i32
            })
            .collect();
        assert!(k > 0);
        let result = Solution::maximum_happiness_sum(happiness, k as i32);
        assert!(result >= 0);
        assert_eq!(result as u64, expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3], 2, 4);
    }

    #[test]
    fn ex2() {
        test(&[1, 1, 1, 1], 2, 1);
    }

    #[test]
    fn ex3() {
        test(&[2, 3, 4, 5], 1, 5);
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[28273512; 1_000_000], 1, 28273512);
    }

    #[test]
    fn my_extreme_ex2() {
        test(&[1; 1_000_000], 1, 1);
    }

    #[test]
    fn my_extreme_ex3() {
        test(&[1; 1_000_000], 1_000_000, 1);
    }

    #[test]
    fn discussion_case1() {
        test(&[28273512], 1, 28273512);
    }

    #[test]
    fn discussion_case2() {
        test(&[37452398, 11922898], 2, 49375295);
    }

    #[test]
    fn discussion_case3() {
        test(
            &[
                38171534, 11972617, 22937647, 58755803, 43031812, 50312742, 92369003, 89945867,
                33907931, 98292666, 80542170, 94143973, 22723963, 25181908, 58694271, 82190854,
                64568538, 60895346, 38521568, 2817851, 26631361, 53479380, 27248588, 36941225,
                32696024, 3456399, 75671627, 79044211, 87809412, 7810193, 8413684, 42745721,
                3054768, 35782175, 53830879, 52176755, 32933410, 40146578, 49400581, 9304944,
                70492709, 26379719, 20636150, 29596181, 26967289, 99829739, 42440279, 72367618,
                4961305, 66113753, 36756455, 77671380, 24555190, 47692123, 32881553, 21021026,
                54482, 41093696, 42586261, 72202549, 93354874, 94954304, 41361448, 76883608,
                89892272, 54015878, 20719316, 71761033, 70452184, 90086614, 43502986, 19670722,
                40636844, 15859405, 57381787, 88816440, 85595546, 27666733, 8439259, 16003018,
                37889020, 67278319, 84517167, 86392695, 54789226, 98212594, 84206190, 16029704,
                32126427, 13524005, 4127867, 63327690, 9162127, 92002186, 29287542, 98634221,
                94771532, 246154, 38390561, 10815442,
            ],
            18,
            1659620952,
        );
    }
}
