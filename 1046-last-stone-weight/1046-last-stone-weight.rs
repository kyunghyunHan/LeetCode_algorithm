impl Solution {
    pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
    loop {
        if stones.len() < 2 {
            break;
        }
        stones.sort();

        let a = stones.remove(stones.len() - 1);
        let b = stones.remove(stones.len() - 1);
        stones.push(a - b);
    }
    stones.len() as i32
}
}