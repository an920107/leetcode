pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let revisions1: Vec<i32> = version1.split('.').map(|s| s.parse().unwrap()).collect();
        let revisions2: Vec<i32> = version2.split('.').map(|s| s.parse().unwrap()).collect();

        let mut i: usize = 0;
        while i < revisions1.len() || i < revisions2.len() {
            let revision1 = revisions1.get(i).unwrap_or(&0);
            let revision2 = revisions2.get(i).unwrap_or(&0);

            if revision1 < revision2 {
                return -1;
            } else if revision1 > revision2 {
                return 1;
            }

            i += 1;
        }
        
        0
    }
}
