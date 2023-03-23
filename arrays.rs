fn main(){
    let arr=[1,2,3,4];

    //travesing array in ust using iterators
    for i in arr.iter() {
        println!("{}",i);
    }

    //traversing array in rust usinf for loop iteration

    for i in 0..arr.len() {
        println!("{}",arr[i]);
    }
}