trait FromRoman {
    fn roman_value(&self) -> i32;
}

impl FromRoman for char {
    fn roman_value(&self) -> i32 {
        match self {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Not a roman numeral")
        } 
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars().rev().fold((0, 0),        // traverse input from right to left accumulating result and remembering previous symbol
            |(sum, prev), c| {
                let n = c.roman_value();    // get numerical value of roman literal
                if n >= prev {              
                    (sum + n, n)            // the "value" of the literal should increase normally
                } else {
                    (sum - n, n)            // if it decreases then substract from the result
                }
            }).0                                 // take sum from the tuple
    }
}