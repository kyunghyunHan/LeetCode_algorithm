impl Solution {
   pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max_v = vec![];
    for i in accounts {
        let mut max_acc = 0;
        for j in i {
            max_acc += j;
        }
        max_v.push(max_acc);
    }
    *max_v.iter().max().unwrap() as i32
}
}