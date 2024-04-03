// https://leetcode.com/problems/word-search/

pub struct Solution;

// Initial over-engineered sol'n
// impl Solution {
//     pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
//         #[derive(Debug)]
//         enum SM {
//             Start,
//             // CheckLeft, // Implicit in Start
//             CheckRight,
//             CheckUp,
//             CheckDown,
//             Done,
//         }
//         // Make sure this struct is 4-aligned for nearly free faster access
//         #[derive(Debug)]
//         #[repr(align(4))]
//         struct State {
//             row: u8,
//             col: u8,
//             index: u8,
//             next_state: SM,
//         }
//         let rows = board.len();
//         assert!(rows > 0);
//         assert!(rows <= 6);
//         let rows = rows as u8;
//         let cols = board[0].len();
//         assert!(cols > 0);
//         assert!(cols <= 6);
//         let cols = cols as u8;
//         let word = word.as_bytes();
//         let mut stack = Vec::new();
//         let mut visited = [[false; 6]; 6];
//         for row in 0..rows {
//             for col in 0..cols {
//                 stack.push(State {
//                     row,
//                     col,
//                     index: 0,
//                     next_state: SM::Start,
//                 });
//                 loop {
//                     let Some(state) = stack.last_mut() else {
//                         break;
//                     };
//                     match state.next_state {
//                         SM::Start => {
//                             if board[state.row as usize][state.col as usize]
//                                 == word[state.index as usize] as char
//                             {
//                                 visited[state.row as usize][state.col as usize] = true;
//                             } else {
//                                 stack.pop();
//                                 continue;
//                             }
//                             let next_row = state.row;
//                             let next_index = state.index + 1;
//                             if next_index == word.len() as u8 {
//                                 return true;
//                             }
//                             state.next_state = SM::CheckRight;
//                             let Some(next_col) = state.col.checked_sub(1) else {
//                                 continue;
//                             };
//                             if !visited[next_row as usize][next_col as usize] {
//                                 stack.push(State {
//                                     row: next_row,
//                                     col: next_col,
//                                     index: next_index,
//                                     next_state: SM::Start,
//                                 });
//                             }
//                         }
//                         SM::CheckRight => {
//                             state.next_state = SM::CheckUp;
//                             let Some(next_col) = state.col.checked_add(1) else {
//                                 continue;
//                             };
//                             if next_col >= cols {
//                                 continue;
//                             }
//                             let next_row = state.row;
//                             let next_index = state.index + 1;
//                             if !visited[next_row as usize][next_col as usize] {
//                                 stack.push(State {
//                                     row: next_row,
//                                     col: next_col,
//                                     index: next_index,
//                                     next_state: SM::Start,
//                                 });
//                             }
//                         }
//                         SM::CheckUp => {
//                             state.next_state = SM::CheckDown;
//                             let Some(next_row) = state.row.checked_sub(1) else {
//                                 continue;
//                             };
//                             let next_col = state.col;
//                             let next_index = state.index + 1;
//                             if !visited[next_row as usize][next_col as usize] {
//                                 stack.push(State {
//                                     row: next_row,
//                                     col: next_col,
//                                     index: next_index,
//                                     next_state: SM::Start,
//                                 });
//                             }
//                         }
//                         SM::CheckDown => {
//                             state.next_state = SM::Done;
//                             let Some(next_row) = state.row.checked_add(1) else {
//                                 continue;
//                             };
//                             if next_row >= rows {
//                                 continue;
//                             }
//                             let next_col = state.col;
//                             let next_index = state.index + 1;
//                             if !visited[next_row as usize][next_col as usize] {
//                                 stack.push(State {
//                                     row: next_row,
//                                     col: next_col,
//                                     index: next_index,
//                                     next_state: SM::Start,
//                                 });
//                             }
//                         }
//                         SM::Done => {
//                             visited[state.row as usize][state.col as usize] = false;
//                             stack.pop();
//                         }
//                     }
//                 }
//             }
//         }
//         false
//     }
// }

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        assert!(rows > 0);
        assert!(rows <= 6);
        let rows = rows as u8;
        let cols = board[0].len();
        assert!(cols > 0);
        assert!(cols <= 6);
        let cols = cols as u8;
        let mut word = word.into_bytes();
        // Check the word can fit in the board
        if word.len() > board.len() * board[0].len() {
            return false;
        }
        // Check all the letters of the word are even in the board
        let mut letter_count = [0; 123 - 65];
        for row in 0..rows {
            for col in 0..cols {
                letter_count[board[row as usize][col as usize] as usize - 65] += 1;
            }
        }
        let should_reverse =
            letter_count[word[0] as usize - 65] > letter_count[word[0] as usize - 65];
        for letter in word.iter() {
            if letter_count[*letter as usize - 65] <= 0 {
                return false;
            }
            letter_count[*letter as usize - 65] -= 1;
        }
        // We don't need this anymore
        let _ = letter_count;
        // Reverse the word if the end has fewer starting letters in the board
        if should_reverse {
            word.reverse();
        }
        // Check if the word can be found in the board
        enum SM {
            Start,
            // CheckLeft, // Implicit in Start
            CheckRight,
            CheckUp,
            CheckDown,
            Done,
        }
        // Make sure this struct is 4-aligned for nearly free faster access
        #[repr(align(4))]
        struct State {
            row: u8,
            col: u8,
            index: u8,
            next_state: SM,
        }
        let mut stack = Vec::new();
        let mut visited = [[false; 6]; 6];
        for row in 0..rows {
            for col in 0..cols {
                if board[row as usize][col as usize] != word[0] as char {
                    continue;
                }
                stack.push(State {
                    row,
                    col,
                    index: 0,
                    next_state: SM::Start,
                });
                loop {
                    let Some(state) = stack.last_mut() else {
                        break;
                    };
                    match state.next_state {
                        SM::Start => {
                            if board[state.row as usize][state.col as usize]
                                == word[state.index as usize] as char
                            {
                                visited[state.row as usize][state.col as usize] = true;
                            } else {
                                stack.pop();
                                continue;
                            }
                            let next_row = state.row;
                            let next_index = state.index + 1;
                            if next_index == word.len() as u8 {
                                return true;
                            }
                            state.next_state = SM::CheckRight;
                            let Some(next_col) = state.col.checked_sub(1) else {
                                continue;
                            };
                            if !visited[next_row as usize][next_col as usize] {
                                stack.push(State {
                                    row: next_row,
                                    col: next_col,
                                    index: next_index,
                                    next_state: SM::Start,
                                });
                            }
                        }
                        SM::CheckRight => {
                            state.next_state = SM::CheckUp;
                            let Some(next_col) = state.col.checked_add(1) else {
                                continue;
                            };
                            if next_col >= cols {
                                continue;
                            }
                            let next_row = state.row;
                            let next_index = state.index + 1;
                            if !visited[next_row as usize][next_col as usize] {
                                stack.push(State {
                                    row: next_row,
                                    col: next_col,
                                    index: next_index,
                                    next_state: SM::Start,
                                });
                            }
                        }
                        SM::CheckUp => {
                            state.next_state = SM::CheckDown;
                            let Some(next_row) = state.row.checked_sub(1) else {
                                continue;
                            };
                            let next_col = state.col;
                            let next_index = state.index + 1;
                            if !visited[next_row as usize][next_col as usize] {
                                stack.push(State {
                                    row: next_row,
                                    col: next_col,
                                    index: next_index,
                                    next_state: SM::Start,
                                });
                            }
                        }
                        SM::CheckDown => {
                            state.next_state = SM::Done;
                            let Some(next_row) = state.row.checked_add(1) else {
                                continue;
                            };
                            if next_row >= rows {
                                continue;
                            }
                            let next_col = state.col;
                            let next_index = state.index + 1;
                            if !visited[next_row as usize][next_col as usize] {
                                stack.push(State {
                                    row: next_row,
                                    col: next_col,
                                    index: next_index,
                                    next_state: SM::Start,
                                });
                            }
                        }
                        SM::Done => {
                            visited[state.row as usize][state.col as usize] = false;
                            stack.pop();
                        }
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        assert_eq!(Solution::exist(board, word), true);
    }

    #[test]
    fn ex2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "SEE".to_string();
        assert_eq!(Solution::exist(board, word), true);
    }

    #[test]
    fn ex3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCB".to_string();
        assert_eq!(Solution::exist(board, word), false);
    }

    #[test]
    fn myex1() {
        let board = vec![vec!['A']];
        let word = "A".to_string();
        assert_eq!(Solution::exist(board, word), true);
    }

    #[test]
    fn myex2() {
        let board = vec![vec!['A']];
        let word = "B".to_string();
        assert_eq!(Solution::exist(board, word), false);
    }

    #[test]
    fn myex3() {
        let board = vec![vec!['A', 'B']];
        let word = "AB".to_string();
        assert_eq!(Solution::exist(board, word), true);
    }

    #[test]
    fn myex4() {
        let board = vec![vec!['A', 'B']];
        let word = "BA".to_string();
        assert_eq!(Solution::exist(board, word), true);
    }

    #[test]
    fn myex5() {
        let board = vec![vec!['A', 'B']];
        let word = "BB".to_string();
        assert_eq!(Solution::exist(board, word), false);
    }

    #[test]
    fn myex6() {
        let board = vec![
            vec!['A', 'B', 'A', 'B', 'A', 'B'],
            vec!['B', 'A', 'B', 'A', 'B', 'A'],
            vec!['A', 'B', 'A', 'B', 'A', 'B'],
            vec!['B', 'A', 'B', 'A', 'B', 'A'],
            vec!['A', 'B', 'A', 'B', 'A', 'B'],
            vec!['B', 'A', 'B', 'A', 'B', 'A'],
        ];
        let word = "ABABABABABABABAD".to_string();
        assert_eq!(Solution::exist(board, word), false);
    }

    #[test]
    fn discussion_case1() {
        let board = vec![
            vec!['A', 'A', 'A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A', 'A', 'B'],
            vec!['A', 'A', 'A', 'A', 'B', 'A'],
        ];
        let word = "AAAAAAAAAAAAABB".to_string();
        assert_eq!(Solution::exist(board, word), false);
    }
}
