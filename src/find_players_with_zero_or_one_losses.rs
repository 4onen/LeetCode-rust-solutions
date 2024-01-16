// https://leetcode.com/problems/find-players-with-zero-or-one-losses/

pub struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut player_losses: std::collections::HashMap<i32, u32> =
            std::collections::HashMap::new();
        for match_ in matches.into_iter() {
            let (winner, loser) = (match_[0], match_[1]);
            player_losses.entry(winner).or_insert(0);
            *player_losses.entry(loser).or_insert(0) += 1
        }
        let mut never_lost = vec![];
        let mut one_loss = vec![];
        for (player, losses) in player_losses {
            match losses {
                0 => never_lost.push(player),
                1 => one_loss.push(player),
                _ => (),
            }
        }
        never_lost.sort_unstable();
        one_loss.sort_unstable();
        vec![never_lost, one_loss]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        let expected = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];
        assert_eq!(Solution::find_winners(matches), expected);
    }

    #[test]
    fn ex2() {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        let expected = vec![vec![1, 2, 5, 6], vec![]];
        assert_eq!(Solution::find_winners(matches), expected);
    }

    #[test]
    fn myex1() {
        // Whoops, this proves my best submission actually can't handle the
        // full 10^5 players. Gotta switch the map to values of u32.
        let max = 100_000;
        let winners: Vec<i32> = (2..=max).into_iter().collect();
        let losers = std::iter::repeat(1);
        let matches: Vec<Vec<i32>> = std::iter::zip(winners.iter(), losers)
            .map(|(&w, l)| vec![w, l])
            .collect();
        let expected = vec![winners, vec![]];
        assert_eq!(Solution::find_winners(matches), expected);
    }
}
