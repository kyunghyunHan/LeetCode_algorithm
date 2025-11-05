impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut cnt = vec![0; 10];
        for &d in &digits {
            cnt[d as usize] += 1;
        }

        let mut res :Vec<i32>= Vec::new();

        for i in 1..=9 {
            if cnt[i] == 0 { continue; }
            cnt[i] -= 1;
            for j in 0..=9 {
                if cnt[j] == 0 { continue; }
                cnt[j] -= 1;
                for k in (0..=8).step_by(2) {
                    if cnt[k] == 0 { continue; }
                    let num = i * 100 + j * 10 + k;
                    res.push(num as i32);
                }

                cnt[j] += 1;
            }

            cnt[i] += 1;
        }

        res
    }
}
