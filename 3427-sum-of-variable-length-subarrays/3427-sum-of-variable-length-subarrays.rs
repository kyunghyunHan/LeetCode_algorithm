impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let mut total =0;
        let mut prefixsum = vec![0];
        for i in &nums{
            prefixsum.push(prefixsum.last().unwrap() + i);
        }

        for (i , x) in nums.into_iter().enumerate(){
            let l = 0.max(i as i32 -x);
            let r = i;
            total +=prefixsum[r + 1] - prefixsum[l as usize];
        }
        total
    }
}