pub struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        Parser::new(&expression)
            .expression()
            .0
            .into_iter()
            .collect()
    }
}

/// ```text
/// <expr>          -> <product>
/// <product>       -> <factor> <product-tail>
/// <product-tail>  -> ε | <factor> <product-tail>
/// <factor>        -> <letter> | "{" <union> "}"
/// <union>         -> <expr> <union-tail>
/// <union-tail>    -> ε | "," <expr> <union-tail>
/// ```
struct Parser<'a> {
    s: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            s: input.as_bytes(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.s.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        if self.pos >= self.s.len() {
            return None;
        }
        let c = self.s[self.pos];
        self.pos += 1;
        Some(c)
    }

    fn expression(&mut self) -> Set {
        self.product()
    }

    fn product(&mut self) -> Set {
        let mut acc = self.factor();
        while matches!(self.peek(), Some(b'a'..=b'z' | b'{')) {
            acc = acc * self.factor();
        }
        acc
    }

    fn factor(&mut self) -> Set {
        match self.bump() {
            Some(c @ b'a'..=b'z') => Set::from(c),
            Some(b'{') => {
                let result = self.union();
                match self.bump() {
                    Some(b'}') => result,
                    c => unreachable!("{:?}", c.map(|c| c as char)),
                }
            }
            c => unreachable!("{:?}", c.map(|c| c as char)),
        }
    }

    fn union(&mut self) -> Set {
        let mut acc = self.expression();
        while self.peek() == Some(b',') {
            self.bump();
            acc = acc + self.expression();
        }
        acc
    }
}

struct Set(BTreeSet<String>);

impl From<u8> for Set {
    fn from(value: u8) -> Self {
        Self([(value as char).to_string()].into_iter().collect())
    }
}

impl std::ops::Mul<Set> for Set {
    type Output = Set;
    fn mul(self, rhs: Set) -> Self::Output {
        if rhs.0.len() == 1 {
            let suffix = rhs.0.into_iter().next().unwrap();
            let result = self
                .0
                .into_iter()
                .map(|mut left| {
                    left.push_str(&suffix);
                    left
                })
                .collect();
            Set(result)
        } else {
            let mut result = BTreeSet::new();
            for left in &self.0 {
                for right in &rhs.0 {
                    let mut s = String::with_capacity(left.len() + right.len());
                    s.push_str(left);
                    s.push_str(right);
                    result.insert(s);
                }
            }
            Set(result)
        }
    }
}

impl std::ops::Add<Set> for Set {
    type Output = Set;
    fn add(self, rhs: Set) -> Self::Output {
        let mut result = self.0;
        result.extend(rhs.0);
        Set(result)
    }
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::brace_expansion_ii("{a,b}{c,{d,e}}".to_string()),
        vec![
            "ac".to_string(),
            "ad".to_string(),
            "ae".to_string(),
            "bc".to_string(),
            "bd".to_string(),
            "be".to_string()
        ]
    );

    assert_eq!(
        Solution::brace_expansion_ii("{{a,z},a{b,c},{ab,z}}".to_string()),
        vec![
            "a".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "z".to_string()
        ]
    );
}
