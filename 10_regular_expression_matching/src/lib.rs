pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let tokens = Self::scan(&p.as_bytes());
        let mut memo = HashMap::new();
        Solution::check(&s.as_bytes(), &tokens, &mut memo)
    }

    fn check(s: &[u8], tokens: &[Token], memo: &mut HashMap<(usize, usize), bool>) -> bool {
        if tokens.is_empty() {
            let result = s.is_empty();
            memo.insert((s.len(), tokens.len()), result);
            return result;
        }

        if let Some(result) = memo.get(&(s.len(), tokens.len())) {
            return *result;
        }

        let (token, left_tokens) = tokens.split_first().unwrap();
        let (c, left_s) = s.split_first().unwrap_or((&0, &[]));
        let c_match = !s.is_empty()
            && match token {
                Token::Character(t_c, _) => *c == *t_c,
                Token::Any(_) => true,
            };

        let result = match token {
            Token::Character(_, Times::One) | Token::Any(Times::One) => {
                c_match && Solution::check(left_s, left_tokens, memo)
            }
            Token::Character(_, Times::ZeroOrMore) | Token::Any(Times::ZeroOrMore) => {
                Solution::check(s, left_tokens, memo)
                    || (c_match && Solution::check(left_s, tokens, memo))
            }
        };

        memo.insert((s.len(), tokens.len()), result);
        result
    }

    fn scan(s: &[u8]) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buffer: Option<u8> = None;

        for &c in s {
            if c == b'.' || (c >= b'a' && c <= b'z') {
                if let Some(buffer) = buffer {
                    tokens.push(Token::new(buffer, Times::One));
                }
                buffer = Some(c);
            } else if c == b'*' {
                if let Some(buffer) = buffer {
                    tokens.push(Token::new(buffer, Times::ZeroOrMore));
                }
                buffer = None;
            }
        }

        if let Some(buffer) = buffer {
            tokens.push(Token::new(buffer, Times::One));
        }

        tokens
    }
}

enum Times {
    One,
    ZeroOrMore,
}

enum Token {
    Character(u8, Times),
    Any(Times),
}

impl Token {
    fn new(c: u8, times: Times) -> Self {
        if c == b'.' {
            Self::Any(times)
        } else {
            Self::Character(c, times)
        }
    }
}
