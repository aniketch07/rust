fn main(){
    let mut num=Vec::new();

    for i in 0..10{
        num.push(i);
        println!("Pushed {}",i);
    }

    returnVec(num);

    let vec1=returnToMain();
    println!("this is returned vctor from main function");

    for i in vec1.iter(){
        println!("{}",i);
    }
}

fn returnVec(num:Vec<i32>){
    println!("this is vctor from other function");
    for i in num.iter(){
        println!("{}",i);
    }

}

fn returnToMain()->Vec<i32>{
    let num=vec![1,2,3,4,5];

   return num;
}