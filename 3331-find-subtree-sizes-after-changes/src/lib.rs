pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_subtree_sizes(parents: Vec<i32>, s: String) -> Vec<i32> {
        let mut tree: Vec<HashSet<usize>> = vec![HashSet::new(); parents.len()];

        for (child, &parent) in parents.iter().enumerate().skip(1) {
            tree[parent as usize].insert(child);
        }

        let labels: Vec<usize> = s.bytes().map(|c| (c - b'a') as usize).collect();
        Self::update_tree(&mut tree, &labels, 0, 0, &mut [None; 26]);

        let mut result = vec![0; parents.len()];
        Self::subtree_size(&mut result, &tree, 0);
        result
    }

    fn update_tree(
        tree: &mut Vec<HashSet<usize>>,
        labels: &[usize],
        parent: usize,
        root: usize,
        ancestors: &mut [Option<usize>; 26],
    ) {
        if let Some(ancestor) = ancestors[labels[root]] {
            tree[parent].remove(&root);
            tree[ancestor].insert(root);
        }

        let original_ancestor = ancestors[labels[root]];
        ancestors[labels[root]] = Some(root);
        for &child in tree[root].clone().iter() {
            Self::update_tree(tree, labels, root, child, ancestors);
        }
        ancestors[labels[root]] = original_ancestor;
    }

    fn subtree_size(result: &mut Vec<i32>, tree: &[HashSet<usize>], root: usize) {
        if tree[root].is_empty() {
            result[root] = 1;
            return;
        }

        let mut count = 0;
        for &child in tree[root].iter() {
            Self::subtree_size(result, tree, child);
            count += result[child];
        }
        result[root] = count + 1;
    }
}
