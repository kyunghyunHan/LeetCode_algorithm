impl Solution {
    pub fn check_record(s: String) -> bool {
        /*
        A': 결석한.
'L': 늦은.
'P': 현재의.
        */
        let mut answer = true;
        let mut a = 0;
        let mut l = 0;
        let s = s.chars().collect::<Vec<char>>();
        for i in 0..s.len(){
           match s[i as usize]{
            'A'=> {
               a+=1; 
               l = 0;
            }
            'L' => {
               l+=1;
            }
            _=>{l= 0;}
           }
            if a >= 2 || l >= 3 {
                return false;
            }
        }
        answer

    }

}