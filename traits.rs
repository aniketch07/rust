struct Person{
    name:String,
    age:i32
}

impl ToString for Person{
    fn to_string(&self) -> String{
        return format!("My name is {} and I am {}", self.name,self.age);
    }
}


fn main(){
    let person1=Person{name: "John".to_string(), age:19};

    println!("{}",person1.to_string());

}