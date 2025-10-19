impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut element_sum = 0;
        let mut digit_sum = 0;

        for num in nums {
            element_sum += num;

            let mut x = num;
            while x > 0 {
                digit_sum += x % 10;
                x /= 10;
            }
        }

        (element_sum - digit_sum).abs()
    }
}