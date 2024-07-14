// https://leetcode.com/problems/number-of-atoms/

pub struct Solution;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        type Idx = u16;
        type Cnt = u32;
        type Atom = [u8; 2];
        fn recursive(formula: &[u8], mut pos: Idx) -> (Idx, std::collections::HashMap<Atom, Cnt>) {
            fn parse_maybe_number(formula: &[u8], start_pos: Idx) -> (Idx, Cnt) {
                let mut end_pos: Idx = start_pos;
                while end_pos < formula.len() as u16 && formula[end_pos as usize].is_ascii_digit() {
                    end_pos += 1
                }
                if end_pos > start_pos {
                    (
                        end_pos,
                        unsafe {
                            std::str::from_utf8_unchecked(
                                &formula[start_pos as usize..end_pos as usize],
                            )
                        }
                        .parse()
                        .unwrap(),
                    )
                } else {
                    (start_pos, 1)
                }
            }
            fn parse_atom(formula: &[u8], start_pos: Idx) -> (Idx, Atom) {
                if start_pos < formula.len() as u16 - 1
                    && formula[start_pos as usize + 1].is_ascii_lowercase()
                {
                    (
                        start_pos + 2,
                        [formula[start_pos as usize], formula[start_pos as usize + 1]],
                    )
                } else {
                    (start_pos + 1, [formula[start_pos as usize], 0])
                }
            }
            let mut counts = std::collections::HashMap::<Atom, Cnt>::new();
            while pos < formula.len() as u16 {
                match formula[pos as usize] {
                    b'A'..=b'Z' => {
                        let (new_pos, atom) = parse_atom(formula, pos);
                        let (new_pos, number) = parse_maybe_number(formula, new_pos);
                        *counts.entry(atom).or_insert(0) += number;
                        pos = new_pos;
                    }
                    b'(' => {
                        let (rec_pos, rec_counts) = recursive(formula, pos + 1);
                        let (new_pos, number) = parse_maybe_number(formula, rec_pos);
                        pos = new_pos;
                        for (key, value) in rec_counts.into_iter() {
                            *counts.entry(key).or_insert(0) += value * number;
                        }
                    }
                    b')' => {
                        pos += 1;
                        break;
                    }
                    _ => unreachable!(),
                }
            }
            (pos, counts)
        }
        let (_, counts) = recursive(formula.as_bytes(), 0);
        let mut keys: Vec<Atom> = counts.keys().copied().collect();
        keys.sort_unstable();
        let mut res = std::vec::Vec::<u8>::new();
        for atom in keys {
            res.push(atom[0]);
            if atom[1] > 0 {
                res.push(atom[1]);
            }
            if counts[&atom] > 1 {
                let count = counts[&atom].to_string();
                res.extend_from_slice(count.as_bytes());
            }
        }
        unsafe { std::string::String::from_utf8_unchecked(res) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(formula: &str, expected: &str) {
        assert!(formula.len() >= 1);
        assert!(formula.len() <= 1000);
        for b in formula.bytes() {
            assert!(
                b.is_ascii_uppercase()
                    || b.is_ascii_lowercase()
                    || b.is_ascii_digit()
                    || b == b'('
                    || b == b')'
            );
        }
        assert!(expected.len() >= 1);
        for b in expected.bytes() {
            assert!(b.is_ascii_uppercase() || b.is_ascii_lowercase() || b.is_ascii_digit());
        }
        assert_eq!(
            Solution::count_of_atoms(formula.to_string()),
            expected.to_string()
        );
    }

    #[test]
    fn ex1() {
        test("H2O", "H2O");
    }

    #[test]
    fn ex2() {
        test("Mg(OH)2", "H2MgO2");
    }

    #[test]
    fn ex3() {
        test("K4(ON(SO3)2)2", "K4N2O14S4");
    }
}
