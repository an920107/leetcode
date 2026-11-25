pub struct ProductOfNumbers {
    prefix_products: Vec<i32>,
    len: usize,
}

impl ProductOfNumbers {
    pub fn new() -> Self {
        Self {
            prefix_products: vec![],
            len: 0,
        }
    }

    pub fn add(&mut self, num: i32) {
        self.len += 1;

        if num == 0 {
            self.prefix_products.clear();
            return;
        }

        if let Some(all_product) = self.prefix_products.last() {
            self.prefix_products.push(*all_product * num as i32);
        } else {
            self.prefix_products.push(num);
        }
    }

    pub fn get_product(&self, k: i32) -> i32 {
        if k as usize > self.prefix_products.len() {
            return 0;
        }

        let all_product = *self.prefix_products.last().unwrap();
        if k as usize == self.prefix_products.len() {
            return all_product as i32;
        }

        let others_product = self.prefix_products[self.prefix_products.len() - 1 - k as usize];
        (all_product / others_product) as i32
    }
}
