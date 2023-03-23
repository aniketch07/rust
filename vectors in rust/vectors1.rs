fn main(){
    let mut vec1=Vec:: new();
    let v = vec![1, 2, 3, 4, 5];
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    vec1.push(4);

    println!("Elements of vector1 are :");  
    for i in vec1{  
             println!("{} ",i);  
    }  

    println!("Elements of vector are :");  
    for i in v{  
             println!("{} ",i);  
    }  
    
}