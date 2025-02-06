// https://leetcode.com/problems/tuple-with-same-product/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn tuple_same_product(mut nums: Vec<i32>) -> i32 {
//         type Idx = usize;
//         nums.sort_unstable();
//         let mut products = std::collections::HashMap::<i32, u16>::new();
//         for i in 0..nums.len() as Idx - 1 {
//             for j in i + 1..nums.len() as Idx {
//                 let product = nums[i as usize] * nums[j as usize];
//                 let count = products.entry(product).or_insert(0);
//                 *count += 1;
//             }
//         }
//         let mut result: i32 = 0;
//         for count in products.into_values() {
//             result += 8 * count as i32 * (count as i32 - 1) / 2;
//         }
//         result as i32
//     }
// }

// Add iterator use, remove sort
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        type Idx = u16;
        let mut products = std::collections::HashMap::<i32, i32>::new();
        for i in 0..nums.len() as Idx - 1 {
            for j in i + 1..nums.len() as Idx {
                let product = nums[i as usize] * nums[j as usize];
                let count = products.entry(product).or_insert(0);
                *count += 1;
            }
        }
        products
            .into_values()
            .map(|count| 4 * count as i32 * (count as i32 - 1))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u16], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 1_000);
        let mut seen = [false; 10_001];
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 10_000);
            assert!(!seen[num as usize]);
            seen[num as usize] = true;
        }
        assert_eq!(
            Solution::tuple_same_product(nums.iter().map(|&x| x as i32).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[2, 3, 4, 6], 8)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 4, 5, 10], 16)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 2, 4, 5, 10, 20], 16 + 24)
    }

    #[test]
    fn discussion_case2() {
        test(&[2, 15, 3, 10, 5, 6], 24)
    }

    #[test]
    fn discussion_case3() {
        test(
            &[
                1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192,
            ],
            1288,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                1331, 5211, 1788, 7178, 1365, 3237, 8211, 8046, 1333, 8167, 8668, 4500, 8153, 7354,
                2745, 5169, 6226, 3801, 6383, 691, 2834, 9452, 9944, 1100, 4364, 6399, 7668, 4165,
                5808, 7592, 8206, 7855, 4391, 6848, 8034, 1661, 2621, 4386, 8792, 3676, 6725, 5305,
                3431, 6106, 8276, 3402, 5812, 5547, 4022, 3436, 604, 9430, 7579, 855, 8604, 9362,
                5477, 3517, 1612, 6339, 2931, 997, 5183, 9547, 642, 6259, 8071, 8540, 5120, 8460,
                5689, 2406, 1781, 714, 9637, 9321, 2482, 1531, 1414, 6074, 1941, 9130, 2449, 9911,
                7007, 5436, 5905, 9121, 1141, 9415, 9800, 7238, 3391, 1492, 9733, 8023, 5538, 7899,
                5638, 2165, 614, 2423, 819, 2742, 4751, 4710, 9929, 2819, 5874, 2053, 1826, 2677,
                5703, 9904, 3493, 4657, 4812, 916, 7553, 8410, 786, 1836, 7588, 2441, 6132, 1411,
                413, 9634, 5035, 6254, 4517, 4625, 8732, 1024, 6464, 7402, 9072, 6222, 1056, 1129,
                4421, 3298, 2119, 7903, 7136, 4082, 8723, 552, 5456, 2489, 3748, 7169, 2106, 9920,
                3599, 1785, 8203, 5779, 5127, 960, 5278, 3547, 7624, 1936, 2395, 7446, 7164, 217,
                470, 8239, 5032, 6218, 5236, 4373, 4503, 3326, 6575, 794, 5875, 8072, 8449, 670,
                8321, 2784,
            ],
            88,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        test(&(1..=1000).into_iter().collect::<Vec<_>>(), 5894072)
    }

    #[test]
    fn my_extreme_ex2() {
        test(
            &(10000 - 999..=10000).into_iter().collect::<Vec<_>>(),
            116464,
        )
    }
}
