// https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/

pub struct Solution;

// Long sol'n
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Player {
//     A,
//     B,
// }
// impl Into<String> for Player {
//     fn into(self) -> String {
//         match self {
//             Player::A => "A".to_string(),
//             Player::B => "B".to_string(),
//         }
//     }
// }
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum GameResult {
//     Draw,
//     Winner(Player),
// }
// impl Into<String> for GameResult {
//     fn into(self) -> String {
//         match self {
//             GameResult::Draw => "Draw".to_string(),
//             GameResult::Winner(player) => player.into(),
//         }
//     }
// }
// struct Board {
//     board: [[Option<Player>; 3]; 3],
// }
// impl Board {
//     fn new() -> Self {
//         Self {
//             board: [[None; 3]; 3],
//         }
//     }
//     fn put(&mut self, player: Player, x: usize, y: usize) {
//         self.board[x][y] = Some(player);
//     }
//     fn result(&self) -> Option<GameResult> {
//         let mut winner = None;
//         for i in 0..3 {
//             if self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2] {
//                 winner = self.board[i][0];
//             }
//             if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] {
//                 winner = self.board[0][i];
//             }
//         }
//         if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
//             winner = self.board[0][0];
//         }
//         if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
//             winner = self.board[0][2];
//         }
//         if let Some(winner) = winner {
//             return Some(GameResult::Winner(winner));
//         }
//         if self
//             .board
//             .iter()
//             .all(|row| row.iter().all(|cell| cell.is_some()))
//         {
//             return Some(GameResult::Draw);
//         }
//         None
//     }
// }
// impl Solution {
//     pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
//         let mut board = Board::new();
//         for (i, m) in moves.into_iter().enumerate() {
//             let player = if i % 2 == 0 { Player::A } else { Player::B };
//             board.put(player, m[0] as usize, m[1] as usize);
//         }
//         board
//             .result()
//             .map(GameResult::into)
//             .unwrap_or_else(|| "Pending".to_string())
//     }
// }

// Short sol'n
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Cell {
            Empty,
            A,
            B,
        }
        use Cell::*;
        let mut board = [[Empty; 3]; 3];
        for (i, m) in moves.into_iter().enumerate() {
            let player = if i % 2 == 0 { A } else { B };
            board[m[0] as usize][m[1] as usize] = player;
        }
        let mut winner = Empty;
        for i in 0..3 {
            if board[i][0] == board[i][1] && board[i][1] == board[i][2] {
                winner = board[i][0];
                break;
            }
            if board[0][i] == board[1][i] && board[1][i] == board[2][i] {
                winner = board[0][i];
                break;
            }
        }
        if winner == Empty && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
            winner = board[0][0];
        }
        if winner == Empty && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
            winner = board[0][2];
        }
        match winner {
            Cell::A => "A".to_string(),
            Cell::B => "B".to_string(),
            Cell::Empty
                if board
                    .iter()
                    .all(|row| row.iter().all(|cell| *cell != Cell::Empty)) =>
            {
                "Draw".to_string()
            }
            _ => "Pending".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let moves = vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]];
        assert_eq!(Solution::tictactoe(moves), "A".to_string());
    }

    #[test]
    fn ex2() {
        let moves = vec![
            vec![0, 0],
            vec![1, 1],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![2, 0],
        ];
        assert_eq!(Solution::tictactoe(moves), "B".to_string());
    }

    #[test]
    fn ex3() {
        let moves = vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 0],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![0, 1],
            vec![0, 2],
            vec![2, 2],
        ];
        assert_eq!(Solution::tictactoe(moves), "Draw".to_string());
    }
}
