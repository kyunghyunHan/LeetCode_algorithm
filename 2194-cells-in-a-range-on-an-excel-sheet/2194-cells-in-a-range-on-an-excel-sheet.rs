impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let parts: Vec<Vec<char>> = s
            .split(':')
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();

        let start_col = parts[0][0] as u8;              
        let end_col   = parts[1][0] as u8;
        let start_row = parts[0][1].to_digit(10).unwrap();
        let end_row   = parts[1][1].to_digit(10).unwrap();

        for c in start_col..=end_col {
            for r in start_row..=end_row {
                result.push(format!("{}{}", c as char, r));
            }
        }

        result
    }
}