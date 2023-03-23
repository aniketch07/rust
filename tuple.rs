fn main(){
    let my_tuple=(1,"aniket",true);
    
    println!("{:?}",my_tuple.0);
    println!("{:?}",my_tuple.1);
    println!("{:?}",my_tuple.2);

    let returnSol=calcualte();
   // let (a,b)=calcualte();
    println!("{:?}",returnSol.0);
    println!("{:?}",returnSol.1);

    
}


fn calcualte()->(i32,bool){

    (42,true)
}