enum Values [derive(Debug)] {
    A(i32),
    B(i32),
    C(String),

}

fn main(){  
    let v = vec![Values::A(5),   
    Values::B(1),Values::C(String::from("javaTpoint"))];  
    for i in v  
   {  
      println!("{:?}",i);  
    }  
}  