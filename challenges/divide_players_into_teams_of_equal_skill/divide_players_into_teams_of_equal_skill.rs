// https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/

pub struct Solution;

// Sorting sol'n
impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        let skill_sum: i64 = skill.iter().map(|&s| s as i64).sum();
        let num_teams = (skill.len() / 2) as i64;
        if skill_sum % num_teams != 0 {
            return -1;
        }
        let target_team_skill = (skill_sum / num_teams) as i32;
        skill.sort_unstable();
        let mut chemistry = 0;
        for i in 0..skill.len() / 2 {
            let team_skill = skill[i] + skill[skill.len() - 1 - i];
            if team_skill != target_team_skill {
                return -1;
            }
            chemistry += (skill[i] * skill[skill.len() - 1 - i]) as i64;
        }
        chemistry
    }
}

// Map sol'n
// impl Solution {
//     pub fn divide_players(mut skill: Vec<i32>) -> i64 {
//         let skill_sum: i64 = skill.iter().map(|&s| s as i64).sum();
//         let target_team_skill = skill_sum / (skill.len() as i64 / 2);
//         // TODO Implement map sol'n
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(skill: &[i32], expected: i64) {
        assert!(skill.len() >= 2);
        assert!(skill.len() <= 100_000);
        assert!(skill.len() % 2 == 0);
        for &s in skill {
            assert!(s >= 1);
            assert!(s <= 1000);
        }
        assert_eq!(Solution::divide_players(skill.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[3, 2, 5, 1, 3, 4], 22)
    }

    #[test]
    fn ex2() {
        test(&[3, 4], 12)
    }

    #[test]
    fn ex3() {
        test(&[1, 1, 2, 3], -1)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[1000; 100_000], (1000 * 1000) * 100_000 / 2)
    }

    #[test]
    fn my_extreme_ex2() {
        let mut skills = [1000; 100_000];
        skills[99_999] = 1;
        test(&skills, -1)
    }

    #[test]
    fn my_extreme_ex3() {
        let mut skills = [1000; 100_000];
        skills[99_999] = 1;
        skills[99_998] = 999;
        skills[99_598] = 999;
        test(&skills, -1)
    }

    #[test]
    fn my_extreme_ex4() {
        let mut skills = [500; 100_000];
        skills[99_999] = 1;
        skills[99_998] = 999;
        skills[99_598] = 499;
        test(&skills, -1)
    }

    #[test]
    fn my_extreme_ex5() {
        let mut skills = [500; 100_000];
        skills[99_999] = 1;
        skills[99_998] = 999;
        skills[99_598] = 499;
        skills[1] = 501;
        test(&skills, 500 * 500 * 99_996 / 2 + 1 * 999 + 499 * 501)
    }
}
