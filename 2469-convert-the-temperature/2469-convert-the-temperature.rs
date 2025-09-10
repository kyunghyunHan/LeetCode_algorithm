impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let kelvin = celsius + 273.15;
        let fahrenheit = celsius * 1.80 + 32.00;
        let result = vec![kelvin,fahrenheit];
        result
    }
}