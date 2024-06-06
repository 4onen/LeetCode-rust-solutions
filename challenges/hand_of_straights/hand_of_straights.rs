// https://leetcode.com/problems/hand-of-straights/

pub struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        assert!(hand.len() > 0);
        assert!(hand.len() <= 10_000);
        assert!(group_size > 0);
        assert!(group_size <= hand.len() as i32);
        let group_size = group_size as u16;
        if hand.len() as u16 % group_size != 0 {
            return false;
        }
        let mut counts = std::collections::HashMap::new();
        let mut unique_cards = std::vec::Vec::new();
        for card in hand {
            *counts.entry(card).or_insert(0) += 1;
            if counts[&card] == 1 {
                unique_cards.push(card);
            }
        }
        unique_cards.sort_unstable();
        for card in unique_cards {
            let count = counts[&card];
            if count == 0 {
                continue;
            }
            for i in 0..group_size {
                let next_card = card + i as i32;
                match counts.get_mut(&next_card) {
                    None => return false,
                    Some(next_count) if *next_count < count => return false,
                    Some(next_count) => {
                        *next_count -= count;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(hand: &[i32], group_size: u16, expected: bool) {
        assert!(hand.len() > 0);
        assert!(hand.len() <= 10_000);
        assert!(group_size > 0);
        assert!(group_size <= hand.len() as u16);
        for &card in hand {
            assert!(card >= 1);
            assert!(card <= 1_000_000_000);
        }
        assert_eq!(
            Solution::is_n_straight_hand(hand.to_vec(), group_size as i32),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3, 6, 2, 3, 4, 7, 8], 3, true)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4, 5], 4, false)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 2, 3, 8, 5, 6, 7, 8], 4, false)
    }

    #[test]
    fn discussion_case2() {
        test(&[2, 1], 2, true)
    }

    #[test]
    fn discussion_case3() {
        test(&[2, 3, 4, 5, 6, 7, 8, 9], 8, true)
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                9, 13, 15, 23, 22, 25, 4, 4, 29, 15, 8, 23, 12, 19, 24, 17, 18, 11, 22, 24, 17, 17,
                10, 23, 21, 18, 14, 18, 7, 6, 3, 6, 19, 11, 16, 11, 12, 13, 8, 26, 17, 20, 13, 19,
                22, 21, 27, 9, 20, 15, 20, 27, 8, 13, 25, 23, 22, 15, 9, 14, 20, 10, 6, 5, 14, 12,
                7, 16, 21, 18, 21, 24, 23, 10, 21, 16, 18, 16, 18, 5, 20, 19, 20, 10, 14, 26, 2, 9,
                19, 12, 28, 17, 5, 7, 25, 22, 16, 17, 21, 11,
            ],
            10,
            false,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                53, 78, 62, 108, 83, 56, 66, 110, 49, 104, 117, 123, 86, 131, 94, 107, 84, 103, 42,
                127, 100, 50, 55, 97, 81, 93, 71, 45, 63, 39, 91, 87, 129, 126, 84, 125, 73, 95,
                116, 47, 106, 52, 121, 54, 38, 68, 69, 76, 89, 90, 57, 67, 86, 114, 64, 87, 79, 92,
                115, 60, 51, 105, 132, 101, 59, 130, 44, 85, 80, 82, 48, 65, 128, 102, 74, 61, 40,
                46, 98, 111, 109, 119, 72, 43, 112, 120, 58, 113, 77, 88, 41, 118, 75, 85, 124,
                122, 96, 83, 99, 70,
            ],
            50,
            true,
        )
    }
}
