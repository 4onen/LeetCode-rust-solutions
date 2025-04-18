// https://leetcode.com/problems/largest-divisible-subset/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
//         nums.sort_unstable();
//         let n = nums.len();
//         let mut dp = vec![1; n];
//         let mut prev = vec![0; n];
//         let mut max_len = 0;
//         let mut max_idx = 0;
//         for i in 0..n {
//             for j in 0..i {
//                 if nums[i] % nums[j] == 0 && dp[i] < dp[j] + 1 {
//                     dp[i] = dp[j] + 1;
//                     prev[i] = j;
//                 }
//             }
//             if dp[i] > max_len {
//                 max_len = dp[i];
//                 max_idx = i;
//             }
//         }
//         let mut res = Vec::new();
//         for _ in 0..max_len {
//             res.push(nums[max_idx]);
//             max_idx = prev[max_idx];
//         }
//         res
//     }
// }

// Optimized sol'n (prealloc capacity)
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut prev = vec![0; n];
        let mut max_len = 0;
        let mut max_idx = 0;
        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[i] < dp[j] + 1 {
                    dp[i] = dp[j] + 1;
                    prev[i] = j;
                }
            }
            if dp[i] > max_len {
                max_len = dp[i];
                max_idx = i;
            }
        }
        let mut res = Vec::with_capacity(max_len as usize);
        for _ in 0..max_len {
            res.push(nums[max_idx]);
            max_idx = prev[max_idx];
        }
        res
    }
}

// Optimized sol'n (use right-size types -- runs slower)
// impl Solution {
//     pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
//         nums.sort_unstable();
//         let n = nums.len() as u16;
//         let mut dp = vec![1; n as usize];
//         let mut prev = vec![0u16; n as usize];
//         let mut max_len = 0;
//         let mut max_idx = 0;
//         for i in 0..n {
//             for j in 0..i {
//                 if nums[i as usize] % nums[j as usize] == 0 && dp[i as usize] < dp[j as usize] + 1 {
//                     dp[i as usize] = dp[j as usize] + 1;
//                     prev[i as usize] = j;
//                 }
//             }
//             if dp[i as usize] > max_len {
//                 max_len = dp[i as usize];
//                 max_idx = i;
//             }
//         }
//         let mut res = Vec::with_capacity(max_len as usize);
//         for _ in 0..max_len {
//             res.push(nums[max_idx as usize]);
//             max_idx = prev[max_idx as usize];
//         }
//         res
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: &[&[i32]]) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 1000);
        assert!(expected.len() >= 1);
        assert!(expected.len() <= nums.len());
        let mut seen = std::collections::HashSet::new();
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 2_000_000_000);
            assert!(seen.insert(num));
        }
        for &answer in expected {
            assert!(answer.is_sorted());
            let mut seen_expected = std::collections::HashSet::new();
            for &num in answer {
                assert!(seen.contains(&num));
                assert!(seen_expected.insert(num));
            }
        }
        let mut res = Solution::largest_divisible_subset(nums.to_vec());
        res.sort_unstable();
        if !expected.contains(&&res[..]) {
            println!("Expected: {:?}", expected);
            println!("Actual: {:?}", res);
            assert!(false);
        }
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3], &[&[1, 2], &[1, 3]]);
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 4, 8], &[&[1, 2, 4, 8]])
    }

    #[test]
    fn discussion_case1() {
        test(&[1462280116], &[&[1462280116]])
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                1, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42,
                44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84,
                86, 88, 90, 92, 94, 96, 98, 100, 102, 104, 106, 108, 110, 112, 114,
            ],
            &[&[1, 2, 4, 8, 16, 32, 64]],
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &[
                282240000, 1, 1975680000, 47040000, 4, 227697153, 1138485765, 1083982858,
                1111782925, 842709010, 721163794, 1702578708, 454906901, 618564121, 28, 976163360,
                1685418020, 1835380773, 1978812972, 1916416565, 829206587, 780026940, 1434884157,
                1016299071, 6720, 168000, 702016067, 1392105552, 1709594705, 110896209, 184347229,
                378555489, 1876985442, 1692894831, 112, 1327454320, 691302516, 1826075259,
                401219197, 1779311230, 1544802429, 1680000, 1538794625, 1404032134, 721049740,
                1270388878, 1741291662, 1039291539, 1851666583, 204983451, 947149980, 1945822879,
                672, 1746637988, 1442521768, 575604905, 1552495279, 1167705265, 50734774,
                1868167352, 1518371519, 895083201, 158472388, 1567207621, 754009800, 1872502475,
                514946764, 1758258895, 1432975061, 202939096, 94251225, 1037065435, 1640781020,
                749252828, 1842597085, 111432420, 737084648, 1331687148, 1437720307, 1044498676,
                977436405, 462247158, 310565110, 1718155513, 802438394, 107440899, 434349315,
                766544646, 1064673551, 537204495, 1791918353, 1178438930, 805633820, 1478523688,
                1985770281, 1971716394, 1098285360, 1434147123, 410195255, 1542954809, 897429305,
                1258177340, 1344, 33600, 840000, 204710723, 884068677, 1909257030, 1420389198,
                694093134, 1105832794, 1685066074, 1954392414, 811756384, 377004900, 1043583846,
                1000669548, 1009185137, 778461554, 737388916, 1890889079, 1526804343, 924525435,
                230434172, 1828472703, 11760000, 1779879296, 757635456, 135307140, 1572206469,
                1534438797, 115682189, 1971440527, 812388239, 1094255505, 1996394383, 35947924,
                554481045, 1488304024, 327231389, 890871201, 1325783457, 1255651748, 1620780452,
                156322215, 303696295, 283763118, 1963388334, 329802162, 1782884790, 753142198,
                899005883, 889956287, 636419010, 1863390660, 18850245, 440255429, 329840589,
                1690639826, 223886803, 792361940, 1049032664, 1900765657, 1248112605, 1937949663,
                179739620, 1892777445, 109934054, 701695976, 924494316, 1075532274, 1304665082,
            ],
            &[&[
                1, 4, 28, 112, 672, 1344, 6720, 33600, 168000, 840000, 1680000, 11760000, 47040000,
                282240000, 1975680000,
            ]],
        )
    }
}
