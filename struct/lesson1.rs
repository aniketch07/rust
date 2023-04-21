#[derive(Debug)]
struct Employee {
    id: i32,
    name: String,
}

fn main() {
    let emp1 = Employee {
        id: 1,
        name:"Aniket".to_string(),
    };

    println!("{:?}", emp1);
}
