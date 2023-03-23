fn main(){

    let person1 = Person{name: "John".to_string(), age:19};

    println!("{:?} {}", person1.speak(),person1.canSpeak());



}


struct Person {
    name: String,
    age: i32

}

trait HasVoiceBox{
    fn speak(&self);

    fn canSpeak(&self) -> bool;
}

impl HasVoiceBox for Person{
    fn speak(&self){
        println!("Hello my name is {}",self.name);
    }

    fn canSpeak(&self) -> bool{
        if self.age>7{
            return true;
        }
        false
        
    }
}