// https://leetcode.com/problems/maximum-score-from-removing-substrings/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
//         let mut s = s.into_bytes();
//         let mut stack = Vec::new();
//         let mut score = 0;
//         let higher_letter = if x > y { b'a' } else { b'b' };
//         let lower_letter = if x > y { b'b' } else { b'a' };
//         let higher_score = if x > y { x } else { y };
//         let lower_score = if x > y { y } else { x };
//         for letter in s.drain(..) {
//             if letter == lower_letter && !stack.is_empty() && *stack.last().unwrap() == higher_letter {
//                 stack.pop();
//                 score += higher_score;
//             } else {
//                 stack.push(letter);
//             }
//         }
//         std::mem::swap(&mut stack, &mut s);
//         for letter in s {
//             if letter == higher_letter && !stack.is_empty() && stack[stack.len() - 1] == lower_letter {
//                 stack.pop();
//                 score += lower_score;
//             } else {
//                 stack.push(letter);
//             }
//         }
//         score
//     }
// }

// Two ptr sol'n v1
// impl Solution {
//     pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
//         assert!(s.len() <= 100_000);
//         const AB: [u8; 2] = [b'a', b'b'];
//         const BA: [u8; 2] = [b'b', b'a'];
//         let mut s = s.into_bytes();
//         let mut score = 0;
//         let (ab, ba) = if x > y { (AB, BA) } else { (BA, AB) };
//         let (higher_score, lower_score) = if x > y { (x, y) } else { (y, x) };
//         let mut read_ptr: u32 = 1;
//         let mut write_ptr: u32 = 1;
//         while read_ptr < s.len() as u32 {
//             if s[read_ptr as usize] == ab[1] && write_ptr > 0 && s[write_ptr  as usize - 1] == ab[0] {
//                 write_ptr -= 1;
//                 score += higher_score;
//             } else {
//                 s[write_ptr as usize] = s[read_ptr as usize];
//                 write_ptr += 1;
//             }
//             read_ptr += 1;
//         }
//         s.truncate(write_ptr as usize);
//         write_ptr = 1;
//         read_ptr = 1;
//         while read_ptr < s.len() as u32 {
//             if s[read_ptr as usize] == ba[1] && write_ptr > 0 && s[write_ptr  as usize- 1] == ba[0] {
//                 write_ptr -= 1;
//                 score += lower_score;
//             } else {
//                 s[write_ptr as usize] = s[read_ptr as usize];
//                 write_ptr += 1;
//             }
//             read_ptr += 1;
//         }
//         score
//     }
// }

// Two ptr sol'n with offload fn
// impl Solution{
//     pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
//         assert!(s.len() <= 100_000);
//         const AB: [u8; 2] = [b'a', b'b'];
//         const BA: [u8; 2] = [b'b', b'a'];
//         fn iterate(s: &mut Vec<u8>, pattern: [u8; 2], score: i32) -> i32 {
//             type Idx = usize;
//             let mut score_out = 0;
//             let mut read_ptr: Idx = 1;
//             let mut write_ptr: Idx = 1;
//             while read_ptr < s.len() as Idx {
//                 if s[read_ptr as usize] == pattern[1] && write_ptr > 0 && s[write_ptr  as usize - 1] == pattern[0] {
//                     write_ptr -= 1;
//                     score_out += score;
//                 } else {
//                     s[write_ptr as usize] = s[read_ptr as usize];
//                     write_ptr += 1;
//                 }
//                 read_ptr += 1;
//             }
//             s.truncate(write_ptr as usize);
//             score_out
//         }
//         let mut s = s.into_bytes();
//         if x > y {
//             iterate(&mut s, AB, x) + iterate(&mut s, BA, y)
//         } else {
//             iterate(&mut s, BA, y) + iterate(&mut s, AB, x)
//         }
//     }
// }

// Two ptr sol'n with offload fn using for loop to iter
impl Solution{
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        assert!(s.len() <= 100_000);
        const AB: [u8; 2] = [b'a', b'b'];
        const BA: [u8; 2] = [b'b', b'a'];
        fn iterate(s: &mut Vec<u8>, pattern: [u8; 2], score: i32) -> i32 {
            type Idx = usize;
            let mut score_out = 0;
            let mut write_ptr: Idx = 1;
            for read_ptr in 1..s.len() {
                s[write_ptr as usize] = s[read_ptr as usize];
                write_ptr += 1;
                if write_ptr > 1 && pattern == [s[write_ptr - 2], s[write_ptr - 1]] {
                    write_ptr -= 2;
                    score_out += score;
                }
            }
            s.truncate(write_ptr as usize);
            score_out
        }
        let mut s = s.into_bytes();
        if x > y {
            iterate(&mut s, AB, x) + iterate(&mut s, BA, y)
        } else {
            iterate(&mut s, BA, y) + iterate(&mut s, AB, x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, x: u16, y: u16, expected: i32) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 100_000);
        assert!(x >= 1);
        assert!(x <= 10_000);
        assert!(y >= 1);
        assert!(y <= 10_000);
        assert_eq!(Solution::maximum_gain(s.to_string(), x as i32, y as i32), expected);
    }

    #[test]
    fn ex1() {
        test("cdbcbbaaabab", 4, 5, 19);
    }

    #[test]
    fn ex2() {
        test("aabbaaxybbaabb", 5, 4, 20);
    }

    #[test]
    fn discussion_case1() {
        test("abbmzgaabtaabsbabhaahabnaeabdbaababbbiabaavababtabwbababzbdabbaaabhbyabdvabbaabbquapaaaaqbbblbuaawlnbbaxaubbbbbpbabbbpaaaacbbaabaaaahbbcoyaauabanqaabpbbbgaawbhabbbbaobsaaababbafbababbbbaaaqbabsbsmabbxqylbbbba", 9421, 8003, 382920);
    }

    #[test]
    fn myex1() {
        test("abab", 1, 1, 2);
    }

    #[test]
    fn myex2() {
        test("abab", 2, 1, 4);
    }

    #[test]
    fn myex3() {
        test("abab", 1, 2, 3);
    }

    #[test]
    fn myex4() {
        test("abab", 2, 2, 4);
    }

    #[test]
    fn myex5() {
        test("abba", 1, 1, 2);
    }

    #[test]
    fn myex6() {
        test("abba", 2, 1, 3);
    }
}
