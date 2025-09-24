impl Solution {
  pub fn make_smallest_palindrome(s: String) -> String {
        let mut a: Vec<char> = s.chars().collect();
        let (mut left, mut right) = (0usize, a.len().saturating_sub(1));

        while left < right {
            if a[left] != a[right] {
                // 더 작은 문자로 양쪽을 맞춤
                let smaller = a[left].min(a[right]);
                a[left] = smaller;
                a[right] = smaller;
            }
            left += 1;
            // underflow 방지 위해 체크
            if right == 0 { break; }
            right -= 1;
        }

        a.into_iter().collect()
    }
}