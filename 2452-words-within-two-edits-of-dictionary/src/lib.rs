pub struct Solution;

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut result = vec![];

        for query in queries.into_iter() {
            for target in dictionary.iter() {
                let mut diff_count = 0;

                for (c_query, c_target) in query.bytes().zip(target.bytes()) {
                    if c_query != c_target {
                        diff_count += 1;
                    }
                }

                if diff_count <= 2 {
                    result.push(query.clone());
                    break;
                }
            }
        }

        result
    }
}
