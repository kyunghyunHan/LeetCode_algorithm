impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = vec![first];
        for i in 0..encoded.len(){
            result.push(result[i] ^ encoded[i])
        }
        result
    }
}