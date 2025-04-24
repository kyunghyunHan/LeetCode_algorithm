impl Solution {
 pub fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();

    let (mut i, mut j) = (a.len() as i32 - 1, b.len() as i32 - 1);
    let (a_bytes, b_bytes) = (a.as_bytes(), b.as_bytes());

    let mut carry = 0;

    while i >= 0 || j >= 0 || carry > 0 {
        let mut sum = carry;

        if i >= 0 {
            sum += (a_bytes[i as usize] - b'0') as u8;
            i -= 1;
        }

        if j >= 0 {
            sum += (b_bytes[j as usize] - b'0') as u8;
            j -= 1;
        }

        result.push((b'0' + (sum % 2)) as char);
        carry = sum / 2;
    }

    result.chars().rev().collect()
}

}