struct Employees{
    name:String,
    id:i32,
    department:String,
}

fn main() {
    // let employee1=Employees{
    //     name:String::from("Aniket"),
    //     id:1,
    //     department:String::from("CSE"),
    // };
    // let employee2=Employees{
    //     name:String::from("Madhav"),
    //     id:2,
    //     department:String::from("CSE"),

    // };

    let emp=new_employees("aniket".to_string(), 1,"CSE".to_string());
    let emp2=new_employees("Yoon".to_string(), 1,"CSE".to_string());
    println!("{}",emp.name);

    // println!("Name: {}", employee1.name);
}

fn new_employees(name:String,id:i32,department:String)->Employees{
    let emp=Employees{
        name:name,
        id:id,
        department:department,
    };
    println!("Name: {}", emp.name);
    emp

   
}