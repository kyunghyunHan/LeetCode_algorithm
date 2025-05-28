impl Solution {
pub fn can_construct(ransom_note: String, mut magazine: String) -> bool {
    let mut result = true;
    let mut cnt = 0;
    let rasome = ransom_note.chars().collect::<Vec<char>>();
    let mut magaz = magazine.chars().collect::<Vec<char>>();

    for i in &rasome {
        if magaz.contains(&i) {
            if let Some(pos) = magaz.iter().position(|&x| x == *i) {
                magaz.remove(pos);
                cnt += 1;

            }
        }
    }

    if cnt == rasome.len() { true } else { false }
}

}