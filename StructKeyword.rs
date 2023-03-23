fn main(){
    let person1=Person{
        name:String::from("Aniket"),
        age:20
    };

    let person2=Person{
        name:String::from("madhav"),
        age:20
    };

    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    println!("Name: {}", person2.name);
    println!("Age: {}", person2.age);
}

struct Person{
    name:String,
    age:i32,
}