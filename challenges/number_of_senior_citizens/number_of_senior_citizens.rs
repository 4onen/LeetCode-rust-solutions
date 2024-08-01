// https://leetcode.com/problems/number-of-senior-citizens/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn count_seniors(details: Vec<String>) -> i32 {
//         details
//             .iter()
//             .filter(|deets| {
//                 let bytes = deets.as_bytes();
//                 let age_tens = bytes[11];
//                 let age_ones = bytes[12];
//                 match (age_tens, age_ones) {
//                     (b'6', b'1'..=b'9') => true,
//                     (b'7'..=b'9', _) => true,
//                     _ => false,
//                 }
//             })
//             .count() as i32
//     }
// }

// Parse sol'n
// impl Solution {
//     pub fn count_seniors(details: Vec<String>) -> i32 {
//         details
//             .iter()
//             .filter(|deets| {
//                 unsafe { std::str::from_utf8_unchecked(&deets.as_bytes()[11..13]) }
//                     .parse::<u8>()
//                     .unwrap() > 60
//             })
//             .count() as i32
//     }
// }

// Parse sol'n without bytes conversion
// impl Solution {
//     pub fn count_seniors(details: Vec<String>) -> i32 {
//         details
//             .iter()
//             .filter(|deets| {
//                 (&deets[11..13]).parse::<u8>().unwrap() > 60
//             })
//             .count() as i32
//     }
// }

// Parse sol'n with raw loop
impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut count = 0;
        for deets in details {
            if (&deets[11..13]).parse::<u8>().unwrap() > 60 {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(details: &[&str], expected: i32) {
        let details = details.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::count_seniors(details), expected);
    }

    #[test]
    fn ex1() {
        test(
            &["7868190130M7522", "5303914400F9211", "9273338290F4010"],
            2,
        )
    }

    #[test]
    fn ex2() {
        test(&["1313579440F2036", "2921522980M5644"], 0)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                "5612624052M0130",
                "5378802576M6424",
                "5447619845F0171",
                "2941701174O9078",
            ],
            2,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                "1137774788O7691",
                "7472708234O0072",
                "9885137889M8868",
                "1321925389M8161",
                "5574709492O7158",
                "2205791488F8896",
                "1537217483M5762",
            ],
            5,
        )
    }
}
