// https://leetcode.com/problems/robot-collisions/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn survived_robots_healths(positions: Vec<i32>, mut healths: Vec<i32>, directions: String) -> Vec<i32> {
        let directions = directions.into_bytes();
        assert!(healths.len() <= 100_000);
        assert_eq!(positions.len(), healths.len());
        assert_eq!(positions.len(), directions.len());
        // Step 1: Multi-sort the robots by position (ignoring health & direction)
        let mut robot_idxs: Vec<u32> = std::vec::Vec::with_capacity(positions.len());
        let cap = robot_idxs.spare_capacity_mut();
        for i in 0..positions.len() as u32 {
            cap[i as usize].write(i);
        }
        unsafe { robot_idxs.set_len(positions.len()); }
        robot_idxs.sort_unstable_by_key(|&i| positions[i as usize]);
        // Step 2: Iterate over the sorted robots, simulating the collisions
        let mut survivors: std::collections::HashSet<u32> = std::collections::HashSet::new();
        {
            let mut right_mover_stack: Vec<u32> = Vec::new();
            for i in robot_idxs {
                let dir = directions[i as usize];
                if dir == b'R' {
                    right_mover_stack.push(i);
                } else {
                    let mut left_mover_health = healths[i as usize];
                    while let Some(j) = right_mover_stack.pop() {
                        let right_mover_health = healths[j as usize];
                        match std::cmp::Ord::cmp(&left_mover_health, &right_mover_health) {
                            std::cmp::Ordering::Less => {
                                healths[j as usize] -= 1;
                                left_mover_health = 0;
                                right_mover_stack.push(j);
                                break;
                            },
                            std::cmp::Ordering::Equal => {
                                left_mover_health = 0;
                                break;
                            },
                            std::cmp::Ordering::Greater => {
                                left_mover_health -= 1;
                            },
                        }
                    }
                    if left_mover_health > 0 && right_mover_stack.is_empty() {
                        healths[i as usize] = left_mover_health;
                        survivors.insert(i);
                    }
                }
            }
            // All right-moving robots at the end are survivors;
            // they have no left-moving robots to collide with.
            for i in right_mover_stack {
                survivors.insert(i);
            }
        }
        // Step 3: Convert the survivors back to the original order
        // This is a simple two-pointer iteration over healths to remove
        // ones we don't want.
        {
            let mut i = 0;
            for j in 0..healths.len() as u32 {
                if survivors.contains(&j) {
                    healths[i as usize] = healths[j as usize];
                    i += 1;
                }
            }
        }
        healths.truncate(survivors.len());
        healths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(positions: &[i32], healths: &[i32], directions: &str, expected: &[i32]) {
        assert_eq!(healths.len(), positions.len());
        assert_eq!(healths.len(), directions.as_bytes().len());
        assert!(healths.len() <= 100_000);
        for (&p, (&h, &d)) in std::iter::zip(positions.iter(), std::iter::zip(healths.iter(), directions.as_bytes())) {
            assert!(p >= 1);
            assert!(p <= 10_000);
            assert!(h >= 1);
            assert!(h <= 10_000);
            assert!(d == b'R' || d == b'L');
        }
        let result = Solution::survived_robots_healths(positions.to_vec(), healths.to_vec(), directions.to_string());
        assert!(result.len() <= healths.len());
        assert_eq!( result, expected);
    }

    #[test]
    fn ex1() {
        test(&[5,4,3,2,1], &[2,17,9,15,10], "RRRRR", &[2,17,9,15,10]);
    }

    #[test]
    fn ex2() {
        test(&[3,6,2,6], &[10,10,15,12], "RLRL", &[14]);
    }

    #[test]
    fn ex3() {
        test(&[1,2,5,6], &[10,10,11,11], "RLRL", &[]);
    }
}
