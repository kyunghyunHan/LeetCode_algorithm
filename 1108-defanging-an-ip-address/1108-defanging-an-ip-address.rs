impl Solution {
   pub fn defang_i_paddr(address: String) -> String {
    let mut s = "".to_string();
    let c = address.chars().collect::<Vec<char>>();
    for i in 0..c.len() {
        if c[i] == '.' {
            s += "[.]";
        } else {
            s += &c[i].to_string();
        }
    }
    s
}
}