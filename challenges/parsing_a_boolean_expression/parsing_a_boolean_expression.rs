// https://leetcode.com/problems/parsing-a-boolean-expression/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack: Vec<u8> = vec![];
        for &b in expression.as_bytes() {
            if b == b')' {
                let mut cur = vec![];
                while let Some(top) = stack.pop() {
                    if top == b'(' {
                        break;
                    }
                    cur.push(top == b't');
                }
                let op = stack.pop().unwrap();
                let res = match op {
                    b'!' => !cur[0],
                    b'&' => cur.iter().all(|&x| x),
                    b'|' => cur.iter().any(|&x| x),
                    _ => unreachable!(),
                };
                stack.push(if res { b't' } else { b'f' });
            } else if b != b',' {
                stack.push(b);
            }
        }
        stack[0] == b't'
    }
}

// top-down-ish sol'n
// impl Solution {
//     pub fn parse_bool_expr(expression: String) -> bool {
//         let expression = expression.as_bytes();
//         let mut stack: Vec<(u8, bool)> = vec![];
//         let mut op = b'|'; // Default to OR of multiple exprs
//         let mut state = false; // Default to false OR expr1 OR expr2 OR ...
//         let mut i = 0;
//         while i < expression.len() {
//             match expression[i] {
//                 b')' => {
//                     let (old_op, old_state) = stack.pop().unwrap();
//                     op = old_op;
//                     state = match op {
//                         b'!' => !state,
//                         b'|' => state || old_state,
//                         b'&' => state && old_state,
//                         _ => unreachable!(),
//                     };
//                 }
//                 new_op @ (b'!' | b'&' | b'|') => {
//                     stack.push((op, state));
//                     op = new_op;
//                     state = op == b'&';
//                     i += 1;
//                     assert_eq!(expression[i], b'(');
//                 }
//                 b'f' => {
//                     state = match op {
//                         b'!' => true,
//                         b'&' => false,
//                         b'|' => state,
//                         _ => unreachable!(),
//                     };
//                 }
//                 b't' => {
//                     state = match op {
//                         b'!' => false,
//                         b'&' => state,
//                         b'|' => true,
//                         _ => unreachable!(),
//                     };
//                 }
//                 b',' => {},
//                 _ => unreachable!("Unexpected byte: {}", expression[i] as char),
//             }
//             i += 1;
//         }
//         state
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(expression: &str, expected: bool) {
        assert_eq!(Solution::parse_bool_expr(expression.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("&(|(f))", false);
    }

    #[test]
    fn ex2() {
        test("|(f,f,f,t)", true);
    }

    #[test]
    fn ex3() {
        test("!(&(f,t))", true);
    }

    #[test]
    fn discussion_case1() {
        test("!(&(!(t),&(f),|(f)))", true);
    }

    #[test]
    fn discussion_case2() {
        test("!(&(!(&(f)),&(t),|(f,f,t)))", false);
    }

    #[test]
    fn discussion_case2_1() {
        test("!(&(t))", false);
    }

    #[test]
    fn discussion_case2_2() {
        test("&(t)", true);
    }

    #[test]
    fn not_chain() {
        let mut expr = [0; 20_000];
        let mut i = 0;
        let mut opens = 0;
        while i < 10_000 {
            expr[i] = b'!';
            i += 1;
            expr[i] = b'(';
            i += 1;
            opens += 1;
        }
        expr[i] = b't';
        i += 1;
        let o = opens % 2 == 0;
        while opens > 0 {
            expr[i] = b')';
            i += 1;
            opens -= 1;
        }
        test(std::str::from_utf8(&expr[..i]).unwrap(), o);
    }

    #[test]
    fn big_discussion_case() {
        test("!(&(&(&(!(|(f,f,t)),&(&(t),&(!(t),!(|(f,f,t)),!(&(f))),|(f,f,t)),&(!(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t)))),&(&(!(t),!(|(f,f,t)),!(&(f))),!(|(f,f,t)),&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))),!(t))),&(&(!(t),!(|(f,f,t)),!(&(f))),!(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t)))),&(!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))),&(&(!(&(f)),&(t),|(f,f,t)),|(t),|(f,f,t)),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))))),!(!(!(&(&(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))),&(&(&(!(&(f)),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(|(f,f,t)),&(t,t,f)),&(f),&(&(t),&(!(t),!(|(f,f,t)),!(&(f))),|(f,f,t)))))))),&(&(&(&(!(&(f)),&(t),|(f,f,t)),|(t),|(f,f,t)),&(&(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t,t,f),&(&(&(t,t,f),|(f,f,t),|(f)),!(&(t)),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))))),&(t,t,f),&(!(!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t)))),&(!(&(f)),&(t),|(f,f,t)),&(&(&(!(&(f)),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(|(f,f,t)),&(t,t,f)),&(f),&(&(t),&(!(t),!(|(f,f,t)),!(&(f))),|(f,f,t))))),&(&(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))),&(&(&(!(&(f)),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(|(f,f,t)),&(t,t,f)),&(f),&(&(t),&(!(t),!(|(f,f,t)),!(&(f))),|(f,f,t))))),!(&(&(t),&(&(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))),&(&(&(!(&(f)),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(|(f,f,t)),&(t,t,f)),&(f),&(&(t),&(!(t),!(|(f,f,t)),!(&(f))),|(f,f,t)))),&(&(&(t,t,f),|(f,f,t),|(f)),!(&(t)),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t)))))),&(&(!(&(!(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t)))),&(&(!(t),!(|(f,f,t)),!(&(f))),!(|(f,f,t)),&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))),!(t))),&(&(t,t,f),&(&(&(t,t,f),|(f,f,t),|(f)),&(&(!(&(f)),&(t),|(f,f,t)),&(t),&(t,t,f)),&(t,t,f)),!(&(&(t),&(&(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))),&(&(&(!(&(f)),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(|(f,f,t)),&(t,t,f)),&(f),&(&(t),&(!(t),!(|(f,f,t)),!(&(f))),|(f,f,t)))),&(&(&(t,t,f),|(f,f,t),|(f)),!(&(t)),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))))))),!(!(&(&(!(t),!(|(f,f,t)),!(&(f))),&(t),!(&(&(t),&(&(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))),&(&(&(!(&(f)),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(|(f,f,t)),&(t,t,f)),&(f),&(&(t),&(!(t),!(|(f,f,t)),!(&(f))),|(f,f,t)))),&(&(&(t,t,f),|(f,f,t),|(f)),!(&(t)),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t)))))))))),!(&(&(!(&(f)),&(t),|(f,f,t)),&(t),&(t,t,f))),&(&(!(t),!(|(f,f,t)),!(&(f))),!(&(f)),&(!(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t)))),&(&(!(t),!(|(f,f,t)),!(&(f))),!(|(f,f,t)),&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))),!(t))))),&(!(!(|(f,f,t))),&(&(|(t),&(&(!(&(f)),&(t),|(f,f,t)),&(t),&(t,t,f)),!(&(&(!(t),!(|(f,f,t)),!(&(f))),!(|(f,f,t)),&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))))),&(&(&(!(|(f,f,t)),&(&(&(&(!(&(f)),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(|(f,f,t)),&(t,t,f)),&(f),&(&(t),&(!(t),!(|(f,f,t)),!(&(f))),|(f,f,t))),&(&(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t,t,f),&(&(&(t,t,f),|(f,f,t),|(f)),!(&(t)),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))))),&(!(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t)))),&(&(!(t),!(|(f,f,t)),!(&(f))),!(|(f,f,t)),&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))),!(t)),!(f)),&(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),|(t),&(!(t),!(|(f,f,t)),!(&(f))))),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t)))),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),!(t)),&(&(&(&(&(!(&(f)),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(|(f,f,t)),&(t,t,f)),&(f),&(&(t),&(!(t),!(|(f,f,t)),!(&(f))),|(f,f,t))),!(|(f,f,t)),&(&(!(&(f)),&(t),|(f,f,t)),|(t),|(f,f,t))),|(f,f,t),&(&(!(&(f)),&(t),|(f,f,t)),&(t),&(t,t,f))),!(&(&(&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))),&(&(&(!(&(f)),|(t),&(!(t),!(|(f,f,t)),!(&(f)))),!(|(f,f,t)),&(t,t,f)),&(f),&(&(t),&(!(t),!(|(f,f,t)),!(&(f))),|(f,f,t)))))),&(&(&(t,t,f),|(f,f,t),|(f)),!(&(t)),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))))),!(&(|(f,f,t),&(&(f),&(!(t),&(f),|(f)),&(!(&(f)),&(t),|(f,f,t))),&(t))))))", true);
    }
}
