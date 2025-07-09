impl Solution {
    pub fn count_points(rings: String) -> i32 {
    let rings = rings.chars().collect::<Vec<char>>();
    let n = rings.len();
    let mut masks = vec![0; 10];
    for i in (0..n).step_by(2) {
        let color = rings[i];

        let rod = rings[i + 1] as i32 - '0' as i32;

        let mut mask_value = 0;

        if color == 'R' {
            mask_value = 1;
        } else if color == 'G' {
            mask_value = 2;
        } else if color == 'B' {
            mask_value = 4;
        }

        masks[rod as usize] |= mask_value;
    }

    let mut count = 0;

    for i in 0..10 {
        if masks[i] == 7 {
            count += 1;
        }
    }
    count
}

}