const MOD: i32 = 1_000_000_007;

pub struct Fancy {
    nums: Vec<i32>,
    next_op_index: Vec<usize>,
    operations: Vec<(Operation, usize)>,
}

#[derive(Clone, Copy)]
enum Operation {
    Add(i32),
    Multiply(i32),
}

impl Fancy {
    pub fn new() -> Self {
        Self {
            nums: vec![],
            next_op_index: vec![],
            operations: vec![],
        }
    }

    pub fn append(&mut self, val: i32) {
        self.nums.push(val);
        self.next_op_index.push(0);
    }

    pub fn add_all(&mut self, inc: i32) {
        self.operations.push((Operation::Add(inc), self.nums.len()));
    }

    pub fn mult_all(&mut self, m: i32) {
        self.operations
            .push((Operation::Multiply(m), self.nums.len()));
    }

    pub fn get_index(&mut self, index: i32) -> i32 {
        let index = index as usize;

        if index as usize >= self.nums.len() {
            return -1;
        }

        let mut num = self.nums[index];
        for operation in self
            .operations
            .get(self.next_op_index[index]..self.operations.len())
            .unwrap_or(&vec![])
            .iter()
            .filter(|(_, len)| index < *len)
            .map(|(op, _)| op)
        {
            match operation {
                Operation::Add(inc) => num = (num + inc) % MOD,
                Operation::Multiply(m) => num = ((num as i64 * *m as i64) % MOD as i64) as i32,
            };
        }

        self.nums[index] = num;
        self.next_op_index[index] = self.operations.len();

        num
    }
}
