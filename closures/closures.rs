fn main() {
    let add=|a,b|a+b;
    let result=add(1,2);
    println!("{}",result);

    let num2=10;
    let add=|a|a+num2;
    let result=add(1);
    println!("{}",result);

}