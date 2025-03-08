impl Solution {
  
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut ans = vec![0i32; n]; // 초기값 0으로 설정된 벡터
    let mut stack = Vec::new(); // (날짜, 온도) 저장하는 스택

    for (cur_day, &cur_temp) in temperatures.iter().enumerate() {
        while let Some(&(prev_day, prev_temp)) = stack.last() {
            if prev_temp < cur_temp {
                stack.pop();
                ans[prev_day] = cur_day as i32 - prev_day as i32;
            } else {
                break;
            }
        }
        stack.push((cur_day, cur_temp)); // (날짜, 온도) 저장
    }

    ans
}
}