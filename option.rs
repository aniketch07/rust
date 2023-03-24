struct Student{
    name: String,
    locker: Option<i32>,
}

fn main(){

    let student1=Student{
        name: String::from("Aniket"),
        locker:None,
    };
    println!("{}", student1.name);
    match student1.locker{
        Some(num) => println!("Locker number {}",num),
        None => println!("No locker available"),
    }

}