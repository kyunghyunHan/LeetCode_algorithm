impl Solution {
    pub fn count_odd_letters(mut n: i32) -> i32 {
           let d = vec![16577, 97, 4672, 648, 1218, 2067, 8464, 2336, 541, 17];
           let mut toggle:i32 = 0;
            
           while n > 0 {
            let digit = n % 10;
            toggle  ^= d[digit as usize];
            n /= 10;
           }
           toggle.count_ones() as i32
    }
        
    }
