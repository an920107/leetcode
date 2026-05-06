pub struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let sum: i32 = nums.iter().sum();
        let mut current: i32 = (0..n).map(|i| i as i32 * nums[i]).sum();
        let mut result = current;

        for i in 1..n {
            let diff = sum - n as i32 * nums[n - i];
            current += diff;
            result = result.max(current);
        }

        result
    }
}

/*

 f(0)   = 0 * a_0     + 1 * a_1     + 2 * a_2 ....  + (n-1) * a_(n-1)
 f(1)   = 0 * a_(n-1) + 1 * a_0     + 2 * a_1 ....  + (n-1) * a_(n-2)
 f(2)   = 0 * a_(n-2) + 1 * a_(n-1) + 2 * a_0 ....
 ...
 f(n-1) = 0 * a_1     + 1 * a_2     + 2 * a_3 ....


 f(1)-f(0) = 0 * (a_(n-1) - a_0) + 1 * (a_0 - a_1) + 2 * (a_1 - a_2) ... + (n-1) * (a_(n-2) - a_(n-1))

 = 0 * a_(n-1) + [ 1 * a_0 + 1 * a_1 + 1 * a_2 ... + 1 * a_(n-2) ] - (n-1) * a_(n-1)

 /// = a_0 + ... a_(n-2) - (n-1) * a_(n-1) = sum - n * a_(n-1)

 /// f(2)-f(1) = a_(n-1) + a_0 + .... a_(n-3) - (n-1) * a_(n-2) = sum - n * a_(n-2)

 diff = f(i)-f(i-1) = sum - n * a_(n-i)
 f(i) = diff + f(i-1)

*/
