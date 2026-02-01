impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut temp: Vec<i32> = nums;
        let first: i32 = temp[0];
        temp.remove(0);
        temp.sort();
        first + temp[0] + temp[1]
    }
}
