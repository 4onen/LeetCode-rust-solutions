// https://leetcode.com/problems/two-best-non-overlapping-events/

pub struct Solution;

impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|event| event[0]);
        let max_future_event_value = {
            let mut max = 0;
            let mut max_future_event_value = vec![0; events.len()];
            for i in (0..events.len()).rev() {
                let value = events[i][2];
                if max < value {
                    max = value;
                }
                max_future_event_value[i] = max;
            }
            max_future_event_value
        };
        let mut max = 0;
        for i in 0..events.len() {
            let end = events[i][1];
            let value = events[i][2];
            if max < value {
                max = value;
            }
            let first_event_starting_after =
                events[i + 1..].partition_point(|event| event[0] <= end) + i + 1;
            if first_event_starting_after >= events.len() {
                continue;
            }
            let pair_value = value + max_future_event_value[first_event_starting_after];
            if max < pair_value {
                max = pair_value;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(events: &[[i32; 3]], expected: i32) {
        assert!(events.len() >= 2);
        assert!(events.len() <= 100_000);
        for event in events {
            assert_eq!(event.len(), 3);
            assert!(event[0] >= 1);
            assert!(event[1] >= event[0]);
            assert!(event[1] <= 1_000_000_000);
            assert!(event[2] >= 1);
            assert!(event[2] <= 1_000_000);
        }
        assert_eq!(
            Solution::max_two_events(events.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[[1, 3, 2], [4, 5, 2], [2, 4, 3]], 4);
    }

    #[test]
    fn ex2() {
        test(&[[1, 3, 2], [4, 5, 2], [1, 5, 5]], 5);
    }

    #[test]
    fn ex3() {
        test(&[[1, 5, 3], [1, 5, 1], [6, 6, 5]], 8);
    }

    #[test]
    fn discussion_case1() {
        test(&[[35, 90, 47], [72, 80, 70]], 70);
    }

    #[test]
    fn discussion_case2() {
        test(&[[1, 3, 5], [3, 7, 10], [6, 7, 4], [4, 8, 2]], 10);
    }

    #[test]
    fn discussion_case3() {
        test(
            &[[63, 87, 45], [97, 100, 32], [10, 83, 53], [51, 61, 16]],
            32 + 53,
        );
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                [6, 6, 6],
                [7, 9, 4],
                [4, 5, 4],
                [3, 7, 9],
                [6, 10, 8],
                [4, 7, 6],
            ],
            12,
        );
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                [91, 100, 42],
                [92, 100, 22],
                [1, 77, 50],
                [66, 97, 90],
                [98, 98, 68],
                [38, 49, 63],
                [64, 72, 97],
            ],
            165,
        );
    }

    #[test]
    fn discussion_case6() {
        test(
            &[
                [4, 9, 7],
                [4, 5, 6],
                [3, 6, 4],
                [6, 6, 5],
                [7, 9, 4],
                [5, 10, 8],
                [4, 9, 10],
                [3, 5, 1],
            ],
            11,
        );
    }

    #[test]
    fn discussion_case7() {
        let input_str = include_str!("discussion_case7.txt");
        let input = input_str[1..input_str.len() - 2]
            .split("],[")
            .map(|x| {
                let mut iter = x.split(',').map(|x| x.parse().unwrap());
                let start = iter.next().unwrap();
                let end = iter.next().unwrap();
                let value = iter.next().unwrap();
                [start, end, value]
            })
            .collect::<Vec<[i32; 3]>>();
        test(&input, 200000);
    }

    // #[test]
    // fn failing_case1() {
    //     test(&[[1, 1, 5], [3, 4, 1], [2, 4, 5], [2, 5, 3], [2, 3, 4]], 10);
    // }

    // #[test]
    // fn failing_case1_1() {
    //     test(&[[1, 1, 5], [2, 4, 5], [2, 5, 3], [2, 3, 4]], 10);
    // }

    #[test]
    fn failing_case1_2() {
        test(&[[1, 1, 5], [2, 4, 5], [2, 5, 3]], 10);
    }

    #[test]
    fn failing_case1_3() {
        test(&[[1, 1, 5], [2, 4, 5]], 10);
    }
}
