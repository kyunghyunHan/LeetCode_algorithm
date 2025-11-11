impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut freq = vec![0;26];
        let mut max_vowel = 0;
        let mut max_conso = 0;

        for c in s.chars(){
            let i = c as usize-'a' as usize;
            freq[i]+=1;

            if c =='a' || c=='e'|| c=='i' || c=='o' ||  c=='u'{
                max_vowel = max_vowel.max(freq[i]);
            }else{
                max_conso = max_conso.max(freq[i]);
            }
        }
        max_vowel+max_conso
    }
}