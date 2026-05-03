pub struct Solution;

use std::collections::{BTreeMap, HashMap, HashSet};

impl Solution {
    pub fn find_all_people(_: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut meeting_groups: BTreeMap<i32, HashMap<i32, Vec<i32>>> = BTreeMap::new();

        for meeting in meetings.iter() {
            let person_x = meeting[0];
            let person_y = meeting[1];
            let time = meeting[2];

            let graph = meeting_groups.entry(time).or_insert(HashMap::new());
            graph.entry(person_x).or_insert(vec![]).push(person_y);
            graph.entry(person_y).or_insert(vec![]).push(person_x);
        }

        let mut people_has_secret: HashSet<i32> = HashSet::from_iter([0, first_person]);
        for (_, group) in meeting_groups {
            let mut visited: HashSet<i32> = HashSet::new();

            for (person, _) in group.iter() {
                let mut people_in_graph: HashSet<i32> = HashSet::new();
                let mut stack: Vec<i32> = vec![*person];
                while let Some(top) = stack.pop() {
                    if visited.contains(&top) {
                        continue;
                    }

                    visited.insert(top);
                    stack.extend(group[&top].iter());
                    people_in_graph.extend(group[&top].iter());
                }

                if people_in_graph
                    .iter()
                    .any(|person| people_has_secret.contains(person))
                {
                    people_has_secret.extend(people_in_graph);
                }
            }
        }

        people_has_secret.into_iter().collect()
    }
}
