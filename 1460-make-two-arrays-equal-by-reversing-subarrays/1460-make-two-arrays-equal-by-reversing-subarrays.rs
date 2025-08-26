use std::thread;

impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
       let handle1 = thread::spawn(move||{
            let mut arr1= target;
              arr1.sort_by(|a,b|{
            a.cmp(&b)
        });
        arr1
        });
         let handle2 =   thread::spawn(move||{
            let mut arr2 = arr;
              arr2.sort_by(|a,b|{
            a.cmp(&b)
        });
        arr2
        });
      let arr1=  handle1.join().unwrap();
      let arr2=  handle2.join().unwrap();
        
        if arr1 == arr2{
            true
        }else{
            false
        }
    }
}