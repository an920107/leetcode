pub struct Solution;

impl Solution {
    pub fn generate_string(rule: String, template: String) -> String {
        let n = rule.len();
        let m = template.len();

        let rule: Vec<bool> = rule.bytes().map(|r| r == b'T').collect();
        let template = template.as_bytes();

        let mut word: Vec<u8> = vec![0; n + m - 1];
        let mut lock: Vec<bool> = vec![false; n + m - 1];

        for (base, &r) in rule.iter().enumerate() {
            if !r {
                continue;
            }
            for offset in 0..m {
                let i = base + offset;
                if word[i] == 0 {
                    word[i] = template[offset];
                    lock[i] = true;
                } else if word[i] != template[offset] {
                    return String::new();
                }
            }
        }

        for i in 0..word.len() {
            if word[i] == 0 {
                word[i] = b'a';
            }
        }

        for (base, &r) in rule.iter().enumerate() {
            if r {
                continue;
            }
            let substr = &word[base..(base + m)];
            if substr == template {
                let mut changed = false;
                for offset in (0..m).rev() {
                    let i = base + offset;
                    if !lock[i] {
                        word[i] = b'b';
                        changed = true;
                        break;
                    }
                }
                if !changed {
                    return String::new();
                }
            }
        }

        String::from_utf8(word).unwrap()
    }
}
