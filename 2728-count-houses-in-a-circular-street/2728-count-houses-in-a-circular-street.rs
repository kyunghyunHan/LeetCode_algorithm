/**
 * Definition for a street.
 * impl Street {
 *     pub fn new(doors: Vec<i32>) -> Self {}
 *     pub fn open_door(&mut self) {}
 *     pub fn close_door(&mut self) {}
 *     pub fn is_door_open(&self) -> bool {}
 *     pub fn move_right(&mut self) {}
 *     pub fn move_left(&mut self) {}
 * }
 */
impl Solution {
    pub fn house_count(mut street: Street, k: i32) -> i32 {
        let mut count  =1;

        for i in 0..k{
            street.close_door();
            street.move_right();
        }

        street.open_door();
        street.move_right();

        while !street.is_door_open(){
            count+=1;
            street.move_right();

        }
        count
    }
}