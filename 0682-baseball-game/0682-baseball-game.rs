impl Solution {
   pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut record: Vec<i32> = Vec::new();

    for op in operations {
        match op.as_str() {
            "C" => {
                record.pop();
            }
            "D" => {
                if let Some(&last) = record.last() {
                    record.push(last * 2);
                }
            }
            "+" => {
                let len = record.len();
                if len >= 2 {
                    let sum = record[len - 1] + record[len - 2];
                    record.push(sum);
                }
            }
            _ => {
                // Parse as number
                if let Ok(num) = op.parse::<i32>() {
                    record.push(num);
                }
            }
        }
    }

    record.iter().sum()
}

}