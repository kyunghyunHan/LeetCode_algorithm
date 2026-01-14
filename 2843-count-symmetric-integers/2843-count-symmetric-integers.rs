impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut count = 0;

        for n in low..=high {
            let s = n.to_string();
            let len = s.len();

            if len % 2 != 0 {
                continue;
            }

            let half = len / 2;
            let (left, right) = s.split_at(half);

            let sum_left: i32 = left.chars().map(|c| c as i32 - '0' as i32).sum();
            let sum_right: i32 = right.chars().map(|c| c as i32 - '0' as i32).sum();

            if sum_left == sum_right {
                count += 1;
            }
        }

        count
    }
}
