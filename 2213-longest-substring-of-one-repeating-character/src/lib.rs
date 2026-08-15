pub struct Solution;

use std::collections::{BTreeMap, BinaryHeap};

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let mut segments_map: BTreeMap<usize, (usize, u8)> = BTreeMap::new();
        let mut len_heap: BinaryHeap<HeapStruct> = BinaryHeap::new();

        let mut last_start = 0;
        let mut last_c = s.bytes().next().unwrap();
        for (index, c) in s.bytes().enumerate() {
            if c != last_c {
                segments_map.insert(last_start, (index, last_c));
                last_start = index;
                last_c = c;
            }
        }
        segments_map.insert(last_start, (s.len(), s.bytes().next_back().unwrap()));

        for (&start, &(end, _c)) in segments_map.iter() {
            len_heap.push(HeapStruct {
                len: end - start,
                start,
                end,
            });
        }

        let mut result = Vec::with_capacity(query_indices.len());
        for (query_c, query_index) in query_characters.bytes().zip(query_indices.into_iter()) {
            let query_index = query_index as usize;
            // 找到包含 query_index 的區間（小於等於 query_index 的最大 key）
            let (&segment_start, &(segment_end, segment_c)) =
                segments_map.range(..=query_index).next_back().unwrap();

            if query_c != segment_c {
                // (1) 移除被修改的舊區間
                segments_map.remove(&segment_start);

                // (2) 拆分：若有左邊殘留區間，加回 map 和 heap
                if segment_start < query_index {
                    segments_map.insert(segment_start, (query_index, segment_c));
                    len_heap.push(HeapStruct {
                        len: query_index - segment_start,
                        start: segment_start,
                        end: query_index,
                    });
                }

                // (3) 拆分：若有右邊殘留區間，加回 map 和 heap
                if query_index + 1 < segment_end {
                    segments_map.insert(query_index + 1, (segment_end, segment_c));
                    len_heap.push(HeapStruct {
                        len: segment_end - (query_index + 1),
                        start: query_index + 1,
                        end: segment_end,
                    });
                }

                // (4) 合併：以 [query_index, query_index + 1) 為基礎向左與向右合併
                let mut new_start = query_index;
                let mut new_end = query_index + 1;

                // 檢查能否與左邊相鄰區間合併
                if let Some((&left_start, &(left_end, left_c))) =
                    segments_map.range(..query_index).next_back()
                {
                    if left_end == query_index && left_c == query_c {
                        new_start = left_start;
                        segments_map.remove(&left_start);
                    }
                }

                // 檢查能否與右邊相鄰區間合併
                if let Some((&right_start, &(right_end, right_c))) =
                    segments_map.get_key_value(&(query_index + 1))
                {
                    if right_c == query_c {
                        new_end = right_end;
                        segments_map.remove(&right_start);
                    }
                }

                // (5) 將合併後的新區間加入 map 與 heap
                segments_map.insert(new_start, (new_end, query_c));
                len_heap.push(HeapStruct {
                    len: new_end - new_start,
                    start: new_start,
                    end: new_end,
                });
            }

            loop {
                let longest = len_heap.peek().unwrap();
                let segment = segments_map.get(&longest.start);
                if let Some(&segment) = segment
                    && segment.0 == longest.end
                {
                    break;
                } else {
                    len_heap.pop();
                }
            }

            result.push(len_heap.peek().unwrap().len as i32);
        }

        result
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HeapStruct {
    len: usize,
    start: usize,
    end: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "babacc".to_string();
        let query_characters = "bcb".to_string();
        let query_indices = vec![1, 3, 3];
        assert_eq!(
            Solution::longest_repeating(s, query_characters, query_indices),
            vec![3, 3, 4]
        );
    }

    #[test]
    fn test_example_2() {
        let s = "abyzz".to_string();
        let query_characters = "aa".to_string();
        let query_indices = vec![2, 1];
        assert_eq!(
            Solution::longest_repeating(s, query_characters, query_indices),
            vec![2, 3]
        );
    }

    #[test]
    fn test_merge_three() {
        // "bab" -> index 1 changed to 'b' -> "bbb"
        let s = "bab".to_string();
        let query_characters = "b".to_string();
        let query_indices = vec![1];
        assert_eq!(
            Solution::longest_repeating(s, query_characters, query_indices),
            vec![3]
        );
    }
}
