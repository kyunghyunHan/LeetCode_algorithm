impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
          let is_straight = check_straight_line(&coordinates);
  is_straight
    }
}

fn check_straight_line(coordinates: &Vec<Vec<i32>>) -> bool {
    if coordinates.len() <= 2 {
        return true; // 점이 2개 이하이면 직선
    }

    let dx = coordinates[1][0] - coordinates[0][0];
    let dy = coordinates[1][1] - coordinates[0][1];

    for i in 2..coordinates.len() {
        let dx_i = coordinates[i][0] - coordinates[i-1][0];
        let dy_i = coordinates[i][1] - coordinates[i-1][1];

        // 크로스 곱으로 방향 확인
        if dx * dy_i != dy * dx_i {
            return false;
        }
    }

    true
}