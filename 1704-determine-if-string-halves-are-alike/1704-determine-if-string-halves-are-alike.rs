use std::thread;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
            let s = s.chars().collect::<Vec<char>>();

    let mid = s.len() / 2;
    let first_half = s[..mid].to_vec();
    let second_half = s[mid..].to_vec();

    let handle1 = thread::spawn(move || {
        let mut cnt = 0;
        for c in first_half {
            match c {
                'a' | 'i' | 'e' | 'o' | 'u' | 'A' | 'I' | 'E' | 'O' | 'U' => {
                    cnt += 1;
                }
                _ => {}
            }
        }
        cnt
    });

    let handle2 = thread::spawn(move || {
        let mut cnt = 0;

        for c in second_half {
            match c {
                'a' | 'i' | 'e' | 'o' | 'u' | 'A' | 'I' | 'E' | 'O' | 'U' => {
                    cnt += 1;
                }
                _ => {}
            }
        }
        cnt
    });

    let res1 = handle1.join().unwrap();
    let res2 = handle2.join().unwrap();

    if res1 ==res2{
        true
    }else{
        false
    }
    }
}