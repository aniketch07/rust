struct Temprature{
    degrees : i32,
}

impl Temprature {

    fn freezing()->Self{
        Self { degrees:1000}
    }
    fn showTemperature(&self){
        println!("Temperature: {}", self.degrees);
    }
}


fn main() {
    let temp=Temprature { degrees: 100};
    temp.showTemperature();
    let cold=Temprature::freezing();
    cold.showTemperature();


}