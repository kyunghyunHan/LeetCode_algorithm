use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn max_students_on_bench(students: Vec<Vec<i32>>) -> i32 {
   let mut bench_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for student in students {
        let student_id = student[0];
        let bench_id = student[1];
        bench_map.entry(bench_id).or_insert_with(HashSet::new).insert(student_id);
    }

    bench_map.values().map(|s| s.len()).max().unwrap_or(0) as i32
    }
}