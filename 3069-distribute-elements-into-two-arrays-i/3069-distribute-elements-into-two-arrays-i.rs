impl Solution {
        pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr1 = Vec::new();
        let mut arr2 = Vec::new();

        arr1.push(nums[0]);
        arr2.push(nums[1]);

        for i in 2..nums.len() {
            let val = nums[i];
            if arr1.last().unwrap() > arr2.last().unwrap() {
                arr1.push(val);
            } else {
                arr2.push(val);
            }
        }

        arr1.extend(arr2);
        arr1
    }
}