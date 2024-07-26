// https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/

pub struct Solution;

// Initial solution (with some unsafe code in tight loops for performance)
impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        type Distance = u16;
        assert!(n >= 2);
        assert!(n <= 100);
        let n = n as u8;
        assert!(distance_threshold >= 1);
        let distance_threshold = distance_threshold as Distance;
        let mut dist = vec![Distance::MAX; n as usize * n as usize];
        // Floyd-Warshall algorithm
        for i in 0..n {
            dist[i as usize * n as usize + i as usize] = 0;
        }
        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            let weight = edge[2] as Distance;
            dist[from * n as usize + to] = weight;
            dist[to * n as usize + from] = weight;
        }
        for iteration in 0..n {
            for from in 0..n {
                for to in 0..n {
                    let from_iteration = from as usize * n as usize + iteration as usize;
                    let iteration_to = iteration as usize * n as usize + to as usize;
                    let added = Distance::saturating_add(dist[from_iteration], dist[iteration_to]);
                    let original =
                        unsafe { dist.get_unchecked_mut(from as usize * n as usize + to as usize) };
                    if *original > added {
                        *original = added;
                    }
                }
            }
        }
        let mut min_count = n;
        let mut min_city = n - 1;
        for i in 0..n {
            let mut count = 0;
            for j in 0..n {
                if i != j && unsafe{*dist.get_unchecked(i as usize * n as usize + j as usize)} <= distance_threshold {
                    count += 1;
                }
            }
            if count <= min_count {
                min_count = count;
                min_city = i;
            }
        }
        min_city as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u8, edges: &[[i32; 3]], distance_threshold: u16, expected: u32) {
        assert!(n >= 2);
        assert!(n <= 100);
        let n = n as i32;
        assert!(edges.len() >= 1);
        assert!(edges.len() <= (n * (n - 1) / 2) as usize); // 4950 for n = 100
        assert!(distance_threshold >= 1);
        assert!(distance_threshold <= 10_000);
        for &[fromi, toi, weighti] in edges {
            assert!(fromi >= 0);
            assert!(fromi < toi);
            assert!(toi < n);
            assert!(weighti >= 1);
            assert!(weighti <= 10_000);
        }
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(
            Solution::find_the_city(n, edges, distance_threshold as i32),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test(4, &[[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]], 4, 3)
    }

    #[test]
    fn ex2() {
        test(
            5,
            &[
                [0, 1, 2],
                [0, 4, 8],
                [1, 2, 3],
                [1, 4, 2],
                [2, 3, 1],
                [3, 4, 1],
            ],
            2,
            0,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        let n = 100i32;
        let edges = (0..n - 1).map(|i| [i, i + 1, 10_000]).collect::<Vec<_>>();
        test(n as u8, &edges, 10_000, 99)
    }
}
