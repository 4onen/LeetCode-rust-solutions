// https://leetcode.com/problems/implement-queue-using-stacks/

struct MyQueue {
    stack: Vec<i32>,
    queue: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack: Vec::new(),
            queue: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.queue.is_empty() {
            while let Some(x) = self.stack.pop() {
                self.queue.push(x);
            }
        }
        self.queue.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.queue.is_empty() {
            while let Some(x) = self.stack.pop() {
                self.queue.push(x);
            }
        }
        *self.queue.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack.is_empty() && self.queue.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Expected {
        No,
        SomeI(i32),
        SomeBool(bool),
    }

    impl From<i32> for Expected {
        fn from(x: i32) -> Self {
            Expected::SomeI(x)
        }
    }

    impl From<bool> for Expected {
        fn from(x: bool) -> Self {
            Expected::SomeBool(x)
        }
    }

    use Expected::*;

    fn do_test(actions: &[&str], params: &[&[i32]], expected: &[Expected]) {
        let mut actions = actions.into_iter();
        let mut params = params.into_iter();
        let mut expected = expected.into_iter();
        assert_eq!(actions.next(), Some(&"MyQueue"));
        let empty_params: &[i32] = &[];
        assert_eq!(params.next(), Some(&empty_params));
        assert_eq!(expected.next(), Some(&No));
        let mut queue = MyQueue::new();

        for ((&action, &param), &expect) in
            std::iter::zip(std::iter::zip(actions, params), expected)
        {
            match action {
                "push" => {
                    assert_eq!(param.len(), 1);
                    queue.push(param[0]);
                    assert_eq!(expect, No);
                }
                "pop" => {
                    assert_eq!(param.len(), 0);
                    assert_eq!(Expected::from(queue.pop()), expect);
                }
                "peek" => {
                    assert_eq!(param.len(), 0);
                    assert_eq!(Expected::from(queue.peek()), expect);
                }
                "empty" => {
                    assert_eq!(param.len(), 0);
                    assert_eq!(Expected::from(queue.empty()), expect);
                }
                _ => panic!("unknown action {}", action),
            }
        }
    }

    #[test]
    fn ex1() {
        do_test(
            &["MyQueue", "push", "push", "peek", "pop", "empty"],
            &[&[], &[1], &[2], &[], &[], &[]],
            &[No, No, No, SomeI(1), SomeI(1), SomeBool(false)],
        );
    }

    #[test]
    #[rustfmt::skip]
    fn discussion_case1() {
        do_test(
            &["MyQueue","empty","push","push","empty","peek","push","push","peek","pop","empty","peek","pop","empty"],
            &[&[],&[],&[1],&[1],&[],&[],&[5],&[5],&[],&[],&[],&[],&[],&[],],
            &[No,SomeBool(true),No,No,SomeBool(false),SomeI(1),No,No,SomeI(1),SomeI(1),SomeBool(false),SomeI(1),SomeI(1),SomeBool(false),],
        );
    }

    #[test]
    #[rustfmt::skip]
    fn discussion_case2() {
        do_test(
            &["MyQueue","push","pop","push","empty","push","pop","peek","push","empty","pop","pop","push","push","peek","empty","empty","empty","pop","pop","push","push","pop","empty","empty","pop","push","empty","push","pop","push","peek","empty","pop","peek","push","empty","push","empty","push","peek","pop","empty","empty","pop","push","pop","pop","empty","push","pop","push","push","empty","empty","empty","peek","peek","peek","empty","pop","empty","pop","push","empty","pop","pop","push","empty","push","pop","empty","pop","push","peek","pop","push","push","push","push","empty","pop","peek","pop","pop","push","peek","pop","pop","push","pop","push","peek","push","empty","push","empty","push","peek","pop","pop"],
            &[&[],&[7],&[],&[8],&[],&[4],&[],&[],&[4],&[],&[],&[],&[5],&[7],&[],&[],&[],&[],&[],&[],&[9],&[8],&[],&[],&[],&[],&[9],&[],&[4],&[],&[5],&[],&[],&[],&[],&[6],&[],&[7],&[],&[5],&[],&[],&[],&[],&[],&[5],&[],&[],&[],&[2],&[],&[7],&[5],&[],&[],&[],&[],&[],&[],&[],&[],&[],&[],&[2],&[],&[],&[],&[8],&[],&[2],&[],&[],&[],&[2],&[],&[],&[1],&[7],&[9],&[5],&[],&[],&[],&[],&[],&[8],&[],&[],&[],&[7],&[],&[6],&[],&[7],&[],&[4],&[],&[2],&[],&[],&[]],
            &[No,No,SomeI(7),No,SomeBool(false),No,SomeI(8),SomeI(4),No,SomeBool(false),SomeI(4),SomeI(4),No,No,SomeI(5),SomeBool(false),SomeBool(false),SomeBool(false),SomeI(5),SomeI(7),No,No,SomeI(9),SomeBool(false),SomeBool(false),SomeI(8),No,SomeBool(false),No,SomeI(9),No,SomeI(4),SomeBool(false),SomeI(4),SomeI(5),No,SomeBool(false),No,SomeBool(false),No,SomeI(5),SomeI(5),SomeBool(false),SomeBool(false),SomeI(6),No,SomeI(7),SomeI(5),SomeBool(false),No,SomeI(5),No,No,SomeBool(false),SomeBool(false),SomeBool(false),SomeI(2),SomeI(2),SomeI(2),SomeBool(false),SomeI(2),SomeBool(false),SomeI(7),No,SomeBool(false),SomeI(5),SomeI(2),No,SomeBool(false),No,SomeI(8),SomeBool(false),SomeI(2),No,SomeI(2),SomeI(2),No,No,No,No,SomeBool(false),SomeI(1),SomeI(7),SomeI(7),SomeI(9),No,SomeI(5),SomeI(5),SomeI(8),No,SomeI(7),No,SomeI(6),No,SomeBool(false),No,SomeBool(false),No,SomeI(6),SomeI(6),SomeI(7)],
        )
    }

    #[test]
    #[rustfmt::skip]
    fn discussion_case3() {
        do_test(
            &["MyQueue","push","push","push","push","push","peek","push","push","push","push","peek","pop","pop","pop","pop","peek","pop","pop","peek","pop"],
            &[&[],&[1],&[4],&[9],&[2],&[2],&[],&[1],&[2],&[7],&[1],&[],&[],&[],&[],&[],&[],&[],&[],&[],&[]],
            &[No,No,No,No,No,No,SomeI(1),No,No,No,No,SomeI(1),SomeI(1),SomeI(4),SomeI(9),SomeI(2),SomeI(2),SomeI(2),SomeI(1),SomeI(2),SomeI(2)]
        )
    }

    #[test]
    #[rustfmt::skip]
    fn discussion_case4() {
        do_test(
            &["MyQueue","push","push","push","push","push","push","push","push","push","push","peek","pop","pop","pop","pop","pop","pop","pop","peek","pop"],
            &[&[],&[5],&[6],&[7],&[6],&[1],&[8],&[2],&[5],&[4],&[7],&[],&[],&[],&[],&[],&[],&[],&[],&[],&[]],
            &[No,No,No,No,No,No,No,No,No,No,No,SomeI(5),SomeI(5),SomeI(6),SomeI(7),SomeI(6),SomeI(1),SomeI(8),SomeI(2),SomeI(5),SomeI(5)],
        )
    }

    #[test]
    #[rustfmt::skip]
    fn discussion_case5() {
        do_test(
            &["MyQueue","push","push","peek","pop","push","empty","pop","pop","push","push","empty","push","empty","pop","pop","peek","peek","peek","empty","empty","push","pop","pop","push","peek","pop","push","empty","pop","push","empty","empty","pop","push","empty","pop","push","empty","pop","push","peek","push","empty","pop","empty","pop","push","pop","push","empty","pop","push","empty","pop","push","pop","push","peek","pop","push","empty","pop","push","pop","push","push","empty","push","push","pop","empty","pop","push","pop","peek","pop","pop","push","peek","peek","push","empty","push","push","empty","empty","peek","empty","push","pop","pop","pop","peek","empty","pop","push","peek","empty","empty","peek"],
            &[&[],&[9],&[3],&[],&[],&[5],&[],&[],&[],&[3],&[8],&[],&[8],&[],&[],&[],&[],&[],&[],&[],&[],&[4],&[],&[],&[4],&[],&[],&[8],&[],&[],&[3],&[],&[],&[],&[4],&[],&[],&[8],&[],&[],&[2],&[],&[6],&[],&[],&[],&[],&[1],&[],&[7],&[],&[],&[8],&[],&[],&[5],&[],&[6],&[],&[],&[3],&[],&[],&[9],&[],&[2],&[6],&[],&[3],&[5],&[],&[],&[],&[6],&[],&[],&[],&[],&[9],&[],&[],&[2],&[],&[3],&[4],&[],&[],&[],&[],&[3],&[],&[],&[],&[],&[],&[],&[1],&[],&[],&[],&[]],
            &[No,No,No,SomeI(9),SomeI(9),No,SomeBool(false),SomeI(3),SomeI(5),No,No,SomeBool(false),No,SomeBool(false),SomeI(3),SomeI(8),SomeI(8),SomeI(8),SomeI(8),SomeBool(false),SomeBool(false),No,SomeI(8),SomeI(4),No,SomeI(4),SomeI(4),No,SomeBool(false),SomeI(8),No,SomeBool(false),SomeBool(false),SomeI(3),No,SomeBool(false),SomeI(4),No,SomeBool(false),SomeI(8),No,SomeI(2),No,SomeBool(false),SomeI(2),SomeBool(false),SomeI(6),No,SomeI(1),No,SomeBool(false),SomeI(7),No,SomeBool(false),SomeI(8),No,SomeI(5),No,SomeI(6),SomeI(6),No,SomeBool(false),SomeI(3),No,SomeI(9),No,No,SomeBool(false),No,No,SomeI(2),SomeBool(false),SomeI(6),No,SomeI(3),SomeI(5),SomeI(5),SomeI(6),No,SomeI(9),SomeI(9),No,SomeBool(false),No,No,SomeBool(false),SomeBool(false),SomeI(9),SomeBool(false),No,SomeI(9),SomeI(2),SomeI(3),SomeI(4),SomeBool(false),SomeI(4),No,SomeI(3),SomeBool(false),SomeBool(false),SomeI(3)],
        )
    }
}
