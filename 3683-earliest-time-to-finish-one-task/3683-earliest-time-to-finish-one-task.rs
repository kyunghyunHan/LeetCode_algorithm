impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        let mut v = Vec::new();
        for task in tasks{
             let task  =task.iter().sum::<i32>();
             v.push(task);
        }
        *v.iter().min().unwrap()
    }
}