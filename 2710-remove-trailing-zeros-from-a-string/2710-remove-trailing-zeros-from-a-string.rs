impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let num = num.chars().collect::<Vec<char>>();
        let mut lens =0;
        for i in (0..num.len()).rev(){
            if num[i]!='0'{
                lens= i;
                break;
            }
        }
        let result = num[0..=lens].iter().collect::<String>();
        result
    }
}