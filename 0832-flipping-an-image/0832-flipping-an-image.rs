impl Solution {
     pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in image.iter_mut() {
            row.reverse();
            for pixel in row.iter_mut() {
                *pixel ^= 1; 
            }
        }
        image
    }
}