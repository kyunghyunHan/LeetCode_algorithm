impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        let mut n2 = n.pow(2);
        let mut n3 = n.pow(3);

        let hex_part = format!("{:X}", n2);

        let base36_chars = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut base36_part = String::new();
        let mut num = n3;

        if num == 0 {
            base36_part.push('0');
        } else {
            let mut digits = Vec::new();
            while num > 0 {
                let remainder = (num % 36) as usize;
                digits.push(base36_chars[remainder] as char);
                num /= 36;
            }
            digits.reverse();
            for ch in digits {
                base36_part.push(ch);
            }
        }

        format!("{}{}", hex_part, base36_part)
    }
}