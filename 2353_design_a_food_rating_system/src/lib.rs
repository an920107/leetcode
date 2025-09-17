use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, PartialEq, Eq)]
struct Cuisine {
    name: String,
    style: String,
    rating: i32,
}

impl Ord for Cuisine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rating
            .cmp(&other.rating)
            .then(other.name.cmp(&self.name))
            .then(self.style.cmp(&other.style))
    }
}

impl PartialOrd for Cuisine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct FoodRatings {
    cuisines: HashMap<String, Cuisine>,
    max_heaps: HashMap<String, BinaryHeap<Cuisine>>,
}

impl FoodRatings {
    pub fn new(foods: Vec<String>, styles: Vec<String>, ratings: Vec<i32>) -> Self {
        let cuisines_iter =
            foods
                .iter()
                .zip(styles.iter())
                .zip(ratings.iter())
                .map(|((name, style), rating)| Cuisine {
                    name: name.clone(),
                    style: style.clone(),
                    rating: *rating,
                });

        let mut max_heaps: HashMap<String, BinaryHeap<Cuisine>> = HashMap::new();
        for cuisine in cuisines_iter.clone() {
            if let Some(heap) = max_heaps.get_mut(&cuisine.style) {
                heap.push(cuisine);
            } else {
                let mut heap = BinaryHeap::new();
                heap.push(cuisine.clone());
                max_heaps.insert(cuisine.style, heap);
            }
        }

        Self {
            cuisines: HashMap::from_iter(
                cuisines_iter.map(|cuisine| (cuisine.name.clone(), cuisine)),
            ),
            max_heaps: max_heaps,
        }
    }

    pub fn change_rating(&mut self, food_name: String, new_rating: i32) {
        if let Some(cuisine) = self.cuisines.get_mut(&food_name) {
            cuisine.rating = new_rating;

            if let Some(heap) = self.max_heaps.get_mut(&cuisine.style) {
                heap.push(cuisine.clone());
            }
        }
    }

    pub fn highest_rated(&mut self, style: String) -> String {
        let mut result = String::new();
        let heap = self.max_heaps.get_mut(&style).unwrap();

        while let Some(highest) = heap.peek() {
            if highest.rating
                != self
                    .cuisines
                    .get(&highest.name)
                    .map(|cuisine| cuisine.rating)
                    .unwrap_or_default()
            {
                heap.pop();
            } else {
                result = highest.name.clone();
                break;
            }
        }

        result
    }
}
