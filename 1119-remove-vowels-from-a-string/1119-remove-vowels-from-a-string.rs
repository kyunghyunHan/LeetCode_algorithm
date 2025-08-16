impl Solution {
    pub fn remove_vowels(mut s: String) -> String {
     s= s.replace("a", "");
     s= s.replace("i", "");
     s= s.replace("o", "");
     s= s.replace("u", "");
     s= s.replace("e", "");
     s
    }
}