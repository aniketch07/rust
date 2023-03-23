fn main(){
    println!("{:?}",add(1,1))
}

fn add<T: std::ops::Add>(a:&T,b:T)->T{
    
   let x=a+b;
    println!("Hello");
    return T
    
}
