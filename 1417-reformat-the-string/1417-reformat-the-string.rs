impl Solution {
fn reformat(s: String) -> String {
    let mut letters = vec![];
    let mut digits = vec![];

    for ch in s.chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else {
            letters.push(ch);
        }
    }

    if (letters.len() as i32 - digits.len() as i32).abs() > 1 {
        return "".to_string();
    }

    let (longer, shorter) = if letters.len() >= digits.len() {
        (letters, digits)
    } else {
        (digits, letters)
    };

    let mut result = String::new();
    let mut l_iter = longer.into_iter();
    let mut s_iter = shorter.into_iter();

    for _ in 0..s.len() {
        if let Some(c) = l_iter.next() {
            result.push(c);
        }
        if let Some(c) = s_iter.next() {
            result.push(c);
        }
    }

    result
}

}