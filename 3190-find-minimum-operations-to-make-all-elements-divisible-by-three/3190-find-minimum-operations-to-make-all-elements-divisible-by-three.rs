impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;

        for num in nums{
            if (num + 1) % 3 ==0 || (num -1) % 3  ==0 {
                cnt+=1;
            }
        }

        cnt
    }
}