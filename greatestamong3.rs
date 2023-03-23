fn main(){
    let a=10;
    let b=20;
    let c=2;
    if a>b && a>c{
        println!("a is greatest:{}",a);
    }
    else if b>c{
        println!("b is greatest:{}",b);
    }
    else{
        println!("c is greatest:{}",c);
    }
}