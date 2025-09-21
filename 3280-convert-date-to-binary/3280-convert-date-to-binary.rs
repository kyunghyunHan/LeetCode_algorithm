impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let mut result = "".to_string();
        let mut date = date.split("-").collect::<Vec<&str>>();
        let year = date[0].parse::<i32>().unwrap();
        let year= format!("{:b}",year);
        let month = date[1].parse::<i32>().unwrap();
        let month= format!("{:b}",month);
        let day = date[2].parse::<i32>().unwrap();
        let day= format!("{:b}",day);

        result = format!("{}-{}-{}",year,month,day);
       
        result


    }
}