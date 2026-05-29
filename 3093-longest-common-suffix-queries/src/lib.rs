pub struct Solution;

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut trie: Vec<TrieNode> = vec![TrieNode {
            container_index: 0,
            children: [None; 26],
        }];

        for (container_index, word) in words_container.iter().enumerate() {
            if word.len() < words_container[trie[0].container_index].len() {
                trie[0].container_index = container_index;
            }

            let mut node_index = 0;
            for c in word.bytes().map(|c| (c - b'a') as usize).rev() {
                if let Some(original_node_index) = trie[node_index].children[c] {
                    let original_node = &trie[original_node_index];
                    let original_word = &words_container[original_node.container_index];
                    if word.len() < original_word.len() {
                        trie[original_node_index].container_index = container_index;
                    }

                    node_index = original_node_index;
                } else {
                    let new_node_index = trie.len();
                    let new_node = TrieNode {
                        container_index,
                        children: [None; 26],
                    };
                    trie[node_index].children[c] = Some(new_node_index);
                    trie.push(new_node);

                    node_index = new_node_index;
                }
            }
        }

        let mut result = vec![trie[0].container_index as i32; words_query.len()];

        for (query_index, word) in words_query.iter().enumerate() {
            let mut node_index = 0;
            for c in word.bytes().map(|c| (c - b'a') as usize).rev() {
                if let Some(next_node_index) = trie[node_index].children[c] {
                    node_index = next_node_index;
                    let node = &trie[node_index];
                    result[query_index] = node.container_index as i32;
                } else {
                    break;
                }
            }
        }

        result
    }
}

#[derive(Debug)]
struct TrieNode {
    container_index: usize,
    children: [Option<usize>; 26],
}

#[test]
fn test_solution() {
    let words_container = vec!["abcd".to_string(), "bcd".to_string(), "xbcd".to_string()];
    let words_query = vec!["cd".to_string(), "bcd".to_string(), "xyz".to_string()];
    assert_eq!(
        Solution::string_indices(words_container, words_query),
        vec![1, 1, 1]
    );
}
