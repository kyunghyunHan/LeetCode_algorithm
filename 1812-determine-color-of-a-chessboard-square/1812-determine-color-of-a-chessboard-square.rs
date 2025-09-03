impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
          let mut cnt = 0;
    for c in coordinates.chars() {
        cnt += c as i32
    }
    println!("{}", cnt);
    let result;
    if cnt % 2 != 0 {
        result = true;
    } else {
        result = false;
    }
    result
    }
}