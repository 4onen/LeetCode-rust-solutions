// https://leetcode.com/problems/find-polygon-with-the-largest-perimeter/

pub struct Solution;

// Braindead solution
// impl Solution {
//     pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
//         nums.sort_unstable();
//         let prefix_sum = nums
//             .iter()
//             .scan(0i64, |acc, &x| {
//                 *acc += x as i64;
//                 Some(*acc)
//             })
//             .collect::<Vec<_>>();
//         for i in (2..nums.len()).into_iter().rev() {
//             if prefix_sum[i - 1] > nums[i] as i64 {
//                 return prefix_sum[i];
//             }
//         }
//         -1
//     }
// }

// Constant space solution
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();
        let mut sum = nums.iter().map(|&x| x as u64).sum::<u64>();
        for i in (2..nums.len()).into_iter().rev() {
            let n = nums[i] as u64;
            sum -= n;
            if sum > n {
                return (sum + n) as i64;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 5]), 15);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]), 12);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 50]), -1);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::largest_perimeter(vec![1, 2, 1]), -1);
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(
            Solution::largest_perimeter(vec![
                300005055, 352368231, 311935527, 315829776, 327065463, 388851949, 319541150,
                397875604, 311309167, 391897750, 366860048, 359976490, 325522439, 390648914,
                359891976, 369105322, 350430086, 398592583, 354559219, 372400239, 344759294,
                379931363, 308829137, 335032174, 336962933, 380797651, 378305476, 336617902,
                393487098, 301391791, 394314232, 387440261, 316040738, 388074503, 396614889,
                331609633, 374723367, 380418460, 349845809, 318514711, 308782485, 308291996,
                375362898, 397542455, 397628325, 392446446, 368662132, 378781533, 372327607,
                378737987
            ]),
            17876942274
        );
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(
            Solution::largest_perimeter(vec![
                422211615, 694256035, 157110403, 534731822, 271753715, 446252281, 910064205,
                793319780, 464850205, 693633544, 538109659, 979225021, 747163805, 326102476,
                78565673, 772106440, 242012149, 459400091, 802631811, 432825130, 8820108,
                258437599, 344777745, 699809126, 414018167, 181125582, 594186601, 944756472,
                569475776, 584430823, 854513318, 244746436, 890623455, 674933446, 23795141,
                680139868, 341577514, 862295600, 100860275, 897213163
            ]),
            20936862075
        );
    }

    #[test]
    fn discussion_case4() {
        assert_eq!(
            Solution::largest_perimeter(vec![
                1, 2, 5, 10, 20, 40, 79, 159, 318, 635, 1270, 2540, 5080, 10161, 20322, 40643,
                81287, 162574, 325147, 650295, 1300590, 2601179, 5202359, 10404717, 20809435,
                41618870, 83237740, 166475480, 332950959, 665901918
            ]),
            -1
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
    }
}
