// https://leetcode.com/problems/second-minimum-time-to-reach-destination/

pub struct Solution;

// Floyd-Warshall way of finding min distances (only works for min)
//     pub fn second_min_traversals(n: u16, edges: Vec<Vec<i32>>) -> u16 {
//         assert!(n >= 2);
//         // Step 1: Find the minimum and 2nd minimum paths from 1 to every node
//         // (Let's just copy Floyd-Warshall from past sol'ns this month.)
//         let mut dist: Vec<[u16; 2]> = vec![[u16::MAX, u16::MAX]; n as usize * n as usize];
//         for edge in edges.iter() {
//             let i = (edge[0] - 1) as usize;
//             let j = (edge[1] - 1) as usize;
//             dist[i * n as usize + j][0] = 1;
//             dist[j * n as usize + i][0] = 1;
//         }
//         for i in 0..n {
//             dist[i as usize * n as usize + i as usize][0] = 0;
//         }
//         for iteration in 0..n {
//             for from in 0..n {
//                 for to in 0..from {
//                     let from_iteration = from as u32 * n as u32 + iteration as u32;
//                     let iteration_to = iteration as u32 * n as u32 + to as u32;
//                     let added = dist[from_iteration as usize][0]
//                         .saturating_add(dist[iteration_to as usize][0]);
//                     let original = &mut dist[from as usize * n as usize + to as usize];
//                     match added.cmp(&original[0]) {
//                         std::cmp::Ordering::Less => {
//                             let new_dist = [added, original[0]];
//                             original[1] = original[0];
//                             original[0] = added;
//                             dist[to as usize * n as usize + from as usize] = new_dist;
//                         }
//                         std::cmp::Ordering::Equal => {}
//                         std::cmp::Ordering::Greater => {
//                             if added < original[1] {
//                                 original[1] = added;
//                                 dist[to as usize * n as usize + from as usize][1] = added;
//                             }
//                         }
//                     }
//                     //println!("{} {} {} {:?}+{}",iteration, from, to, original, added);
//                 }
//             }
//         }
//         dbg!(&dist);
//         // Step 2: If there is only 1 minimum path, add 2 traversals to its
//         // length to emulate going backward then forward anywhere along it.
//         let minimums = dist[n as usize - 1];
//         if minimums[1] == u16::MAX {
//             minimums[0] + 2
//         } else {
//             minimums[1]
//         }
//     }

impl Solution {
    pub const fn simulate_lights(traversals: u16, time: u16, change: u16) -> i32 {
        let mut second_min_time: i32 = 0 as i32;
        let mut next_stop_signal: i32 = change as i32;
        let mut i = 0;
        while i < traversals {
            if second_min_time >= next_stop_signal {
                let cycle = 2 * change as i32;
                if second_min_time % cycle >= change as i32 {
                    let changes = second_min_time / cycle + 1;
                    second_min_time = changes * cycle;
                    next_stop_signal = second_min_time + (change as i32);
                }
            }
            second_min_time += time as i32;
            i += 1;
        }
        second_min_time
    }
    // Breadth-first search to find the 2nd minimum path
    pub fn second_min_traversals(n: u16, edges: Vec<Vec<i32>>) -> u16 {
        assert!(n >= 2);
        let mut graph: Vec<Vec<u16>> = vec![vec![]; n as usize];
        for edge in edges.iter() {
            let i = (edge[0] - 1) as u16;
            let j = (edge[1] - 1) as u16;
            graph[i as usize].push(j);
            graph[j as usize].push(i);
        }
        let mut going_to = std::vec::Vec::new();
        let mut will_go_to = std::vec::Vec::new();
        let mut dist: Vec<_> = vec![[u16::MAX; 2]; n as usize];
        going_to.push((0,0));
        while !going_to.is_empty() {
            for (node,dist_at) in going_to.drain(..) {
                for &neighbor in graph[node as usize].iter() {
                    if dist_at + 1 < dist[neighbor as usize][0] {
                        dist[neighbor as usize][0] = dist_at+1;
                        will_go_to.push((neighbor,dist_at+1));
                    } else if dist[neighbor as usize][0] < dist_at + 1
                        && dist_at + 1 < dist[neighbor as usize][1]
                    {
                        dist[neighbor as usize][1] = dist_at + 1;
                        if neighbor == n-1 {
                            return dist_at+1;
                        }
                        will_go_to.push((neighbor, dist_at+1));
                    }
                }
            }
            std::mem::swap(&mut going_to, &mut will_go_to);
        }
        unreachable!()
        //dist[(n - 1) as usize][0] + 2
    }
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        assert!(n >= 2);
        let n = n as u16;
        let second_min_traversals = Solution::second_min_traversals(n, edges);
        Solution::simulate_lights(second_min_traversals, time as u16, change as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u16, edges: &[[u16; 2]], time: u16, change: u16, expected: i32) {
        assert!(n >= 2);
        assert!(n <= 10_000);
        assert!(edges.len() >= n as usize - 1);
        assert!(edges.len() <= std::cmp::min(20_000, n as usize * (n as usize - 1) / 2));
        for edge in edges.iter() {
            assert!(edge[0] >= 1);
            assert!(edge[0] <= n);
            assert!(edge[1] >= 1);
            assert!(edge[1] <= n);
            assert!(edge[0] != edge[1]);
        }
        assert!(time >= 1);
        assert!(time <= 1000);
        assert!(change >= 1);
        assert!(change <= 1000);
        assert!(expected > 0);
        let edges = edges
            .iter()
            .map(|e| e.iter().map(|&x| x as i32).collect())
            .collect();
        assert_eq!(
            Solution::second_minimum(n as i32, edges, time as i32, change as i32),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test(5, &[[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]], 3, 5, 13)
    }

    #[test]
    fn ex2() {
        test(2, &[[1, 2]], 3, 2, 11)
    }

    #[test]
    fn discussion_case1() {
        test(
            7,
            &[
                [1, 2],
                [2, 3],
                [3, 4],
                [4, 5],
                [5, 6],
                [6, 7],
                [1, 3],
                [1, 4],
            ],
            1,
            3,
            8,
        )
    }

    #[test]
    fn discussion_case2() {
        test(4, &[[1, 2], [2, 3], [3, 4], [4, 1], [2, 4]], 10, 15, 20)
    }

    #[test]
    fn discussion_case3() {
        test(
            7,
            &[
                [1, 2],
                [2, 3],
                [3, 4],
                [4, 5],
                [5, 6],
                [6, 7],
                [1, 3],
                [1, 4],
            ],
            1,
            3,
            8,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            20,
            &[
                [1, 2],
                [1, 3],
                [1, 4],
                [1, 5],
                [2, 6],
                [2, 7],
                [3, 8],
                [3, 9],
                [4, 10],
                [4, 11],
                [5, 12],
                [5, 13],
                [6, 14],
                [6, 15],
                [7, 16],
                [7, 17],
                [8, 18],
                [9, 19],
                [10, 20],
                [11, 20],
                [12, 19],
                [13, 18],
                [14, 17],
                [15, 16],
                [16, 19],
                [17, 20],
            ],
            10,
            50,
            40,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            25,
            &[
                [1, 2],
                [1, 3],
                [1, 4],
                [2, 5],
                [2, 6],
                [3, 7],
                [3, 8],
                [4, 9],
                [4, 10],
                [5, 11],
                [5, 12],
                [6, 13],
                [6, 14],
                [7, 15],
                [7, 16],
                [8, 17],
                [8, 18],
                [9, 19],
                [9, 20],
                [10, 21],
                [10, 22],
                [11, 23],
                [11, 24],
                [12, 25],
                [13, 24],
                [14, 25],
                [15, 22],
                [16, 21],
                [17, 20],
                [18, 19],
                [19, 25],
                [20, 25],
            ],
            12,
            35,
            94,
        )
    }

    #[test]
    fn discussion_case6() {
        test(
            100,
            &[
                [1, 2],
                [1, 3],
                [1, 4],
                [1, 5],
                [2, 6],
                [2, 7],
                [3, 8],
                [3, 9],
                [4, 10],
                [4, 11],
                [5, 12],
                [5, 13],
                [6, 14],
                [6, 15],
                [7, 16],
                [7, 17],
                [8, 18],
                [8, 19],
                [9, 20],
                [9, 21],
                [10, 22],
                [10, 23],
                [11, 24],
                [11, 25],
                [12, 26],
                [12, 27],
                [13, 28],
                [13, 29],
                [14, 30],
                [14, 31],
                [15, 32],
                [15, 33],
                [16, 34],
                [16, 35],
                [17, 36],
                [17, 37],
                [18, 38],
                [18, 39],
                [19, 40],
                [19, 41],
                [20, 42],
                [20, 43],
                [21, 44],
                [21, 45],
                [22, 46],
                [22, 47],
                [23, 48],
                [23, 49],
                [24, 50],
                [24, 51],
                [25, 52],
                [25, 53],
                [26, 54],
                [26, 55],
                [27, 56],
                [27, 57],
                [28, 58],
                [28, 59],
                [29, 60],
                [29, 61],
                [30, 62],
                [30, 63],
                [31, 64],
                [31, 65],
                [32, 66],
                [32, 67],
                [33, 68],
                [33, 69],
                [34, 70],
                [34, 71],
                [35, 72],
                [35, 73],
                [36, 74],
                [36, 75],
                [37, 76],
                [37, 77],
                [38, 78],
                [38, 79],
                [39, 80],
                [39, 81],
                [40, 82],
                [40, 83],
                [41, 84],
                [41, 85],
                [42, 86],
                [42, 87],
                [43, 88],
                [43, 89],
                [44, 90],
                [44, 91],
                [45, 92],
                [45, 93],
                [46, 94],
                [46, 95],
                [47, 96],
                [47, 97],
                [48, 98],
                [48, 99],
                [49, 100],
                [50, 100],
                [51, 100],
                [52, 100],
                [53, 100],
                [54, 100],
                [55, 100],
                [56, 100],
                [57, 100],
                [58, 100],
                [59, 100],
                [60, 100],
                [61, 100],
                [62, 100],
                [63, 100],
                [64, 100],
                [65, 100],
                [66, 100],
                [67, 100],
                [68, 100],
                [69, 100],
                [70, 100],
                [71, 100],
                [72, 100],
                [73, 100],
                [74, 100],
                [75, 100],
                [76, 100],
                [77, 100],
                [78, 100],
                [79, 100],
                [80, 100],
                [81, 100],
                [82, 100],
                [83, 100],
                [84, 100],
                [85, 100],
                [86, 100],
                [87, 100],
                [88, 100],
                [89, 100],
                [90, 100],
                [91, 100],
                [92, 100],
                [93, 100],
                [94, 100],
                [95, 100],
                [96, 100],
                [97, 100],
                [98, 100],
                [99, 100],
            ],
            15,
            100,
            90,
        )
    }

    #[test]
    fn discussion_case7() {
        let input_str = include_str!("discussion_case7.txt");
        let inputs = input_str
            .trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split("],[")
            .map(|x| {
                let mut nums = x.split(",").map(|y| y.parse::<u16>().unwrap());
                let a = nums.next().unwrap();
                let b = nums.next().unwrap();
                [a, b]
            })
            .collect::<Vec<_>>();
        test(1000, &inputs, 358, 932, 15270)
    }

    #[test]
    fn discussion_case8() {
        let input_str = include_str!("discussion_case8.txt");
        let inputs = input_str
            .trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split("],[")
            .map(|x| {
                let mut nums = x.split(",").map(|y| y.parse::<u16>().unwrap());
                let a = nums.next().unwrap();
                let b = nums.next().unwrap();
                [a, b]
            })
            .collect::<Vec<_>>();
        test(1000, &inputs, 500, 867, 21308)
    }

    #[test]
    fn failing_case1() {
        let input_str = include_str!("failing_case1.txt");
        let inputs = input_str
            .trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split("],[")
            .map(|x| {
                let mut nums = x.split(",").map(|y| y.parse::<u16>().unwrap());
                let a = nums.next().unwrap();
                let b = nums.next().unwrap();
                [a, b]
            })
            .collect::<Vec<_>>();
        test(520, &inputs, 946, 183, 3142)
    }

    #[test]
    fn discussion_case9() {
        // 1 -> 2 -> 5 -> 8
        // 1 -> 3 -> 5 -> 8
        // 1 -> 4 -> 5 -> 8
        test(
            8,
            &[
                [1, 2],
                [1, 3],
                [1, 4],
                [1, 6],
                [2, 5],
                [3, 5],
                [4, 5],
                [5, 8],
                [6, 7],
            ],
            3,
            3,
            27,
        )
    }

    #[test]
    fn discussion_case10() {
        // 1 - 3 - 7 - 8
        // 1 - 3 - 4 - 8
        // 1 - 3 - 4 - 2 - 8
        test(
            8,
            &[
                [4, 3],
                [2, 4],
                [3, 7],
                [2, 8],
                [7, 5],
                [2, 6],
                [3, 1],
                [6, 3],
                [4, 8],
                [8, 7],
            ],
            5,
            2,
            22,
        )
    }

    #[test]
    fn tle_case1() {
        let input_str = include_str!("tle_case1.txt");
        let inputs = input_str
            .trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split("],[")
            .map(|x| {
                let mut nums = x.split(",").map(|y| y.parse::<u16>().unwrap());
                let a = nums.next().unwrap();
                let b = nums.next().unwrap();
                [a, b]
            })
            .collect::<Vec<_>>();
        test(10000, &inputs, 1000, 1000, 19717000)
    }

    fn test_traversals(n: u16, edges: &[[u16; 2]], expected: u16) {
        assert!(n >= 2);
        assert!(n <= 1000);
        assert!(edges.len() >= n as usize - 1);
        assert!(edges.len() <= std::cmp::min(20_000, n as usize * (n as usize - 1) / 2));
        for edge in edges.iter() {
            assert!(edge[0] >= 1);
            assert!(edge[0] <= n);
            assert!(edge[1] >= 1);
            assert!(edge[1] <= n);
            assert!(edge[0] != edge[1]);
        }
        assert!(expected > 0);
        let edges = edges
            .iter()
            .map(|e| e.iter().map(|&x| x as i32).collect())
            .collect();
        assert_eq!(Solution::second_min_traversals(n, edges), expected);
    }

    #[test]
    fn ex1_traversals() {
        test_traversals(5, &[[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]], 3);
    }

    #[test]
    fn ex2_traversals() {
        test_traversals(2, &[[1, 2]], 3);
    }

    #[test]
    fn myex1_traversals() {
        test_traversals(4, &[[1, 4], [1, 2], [2, 3], [3, 4]], 3);
    }

    #[test]
    fn myex2_traversals() {
        test_traversals(4, &[[2, 4], [1, 2], [2, 3], [3, 4]], 3);
    }

    #[test]
    fn myex3_traversals() {
        test_traversals(5, &[[2, 5], [1, 2], [2, 3], [3, 4], [4, 5]], 4);
    }

    #[test]
    fn myex4_traversals() {
        test_traversals(6, &[[2, 5], [1, 2], [2, 3], [3, 4], [4, 5], [5, 6]], 5);
    }

    #[test]
    fn discussion_case9_traversals() {
        // 1 -> 2 -> 5 -> 8
        // 1 -> 3 -> 5 -> 8
        // 1 -> 4 -> 5 -> 8
        test_traversals(
            8,
            &[
                [1, 2],
                [1, 3],
                [1, 4],
                [1, 6],
                [2, 5],
                [3, 5],
                [4, 5],
                [5, 8],
                [6, 7],
            ],
            5,
        )
    }

    #[test]
    fn discussion_case10_traversals() {
        // 1 - 3 - 7 - 8
        // 1 - 3 - 4 - 8
        // 1 - 3 - 4 - 2 - 8
        test_traversals(
            8,
            &[
                [4, 3],
                [2, 4],
                [3, 7],
                [2, 8],
                [7, 5],
                [2, 6],
                [3, 1],
                [6, 3],
                [4, 8],
                [8, 7],
            ],
            4,
        )
    }

    fn test_lights(traversals: u16, time: u16, change: u16, expected: i32) {
        assert!(traversals >= 1);
        assert!(traversals <= 1000);
        assert!(time >= 1);
        assert!(time <= 1000);
        assert!(change >= 1);
        assert!(change <= 1000);
        assert!(expected > 0);
        assert_eq!(
            Solution::simulate_lights(traversals, time, change),
            expected
        );
    }

    #[test]
    fn ex1_lights() {
        test_lights(3, 3, 5, 13);
    }

    #[test]
    fn ex2_lights() {
        test_lights(3, 3, 2, 11);
    }

    #[test]
    fn discussion_case9_lights() {
        test_lights(5, 3, 3, 27)
    }

    #[test]
    fn discussion_case10_lights() {
        // 1 - 3 - 7 - 8
        // 1 - 3 - 4 - 8
        // 1 - 3 - 4 - 2 - 8
        test_lights(4, 5, 2, 22)
    }

    #[test]
    fn my_lights_ex1() {
        test_lights(2, 7, 4, 15)
    }
}
