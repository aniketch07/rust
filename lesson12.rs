fn add<T:std::ops::Add<Output=T>>(a:T,b:T)->T{
    a+b
}

fn print_result(res:i32){
    println!("{:?}",res);
}

fn main(){
    let res=add(2,3);
    print_result(res);
}