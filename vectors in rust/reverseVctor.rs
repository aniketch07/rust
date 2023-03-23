fn main(){
    let arr=vec![11,24,5,13,9];


    for i in (0..arr.len()).rev() {
        println!("{}", arr[i]);
    }

    //sum of elements in arr

    let mut sum=0;

    for i in 0..arr.len() {
        sum=sum + arr[i];
    }

    println!("{}", sum);

    //smallest element

    let mut min=arr[0];

    for i in 0..arr.len() {
        if arr[i]<min {
            min=arr[i];
        }
    }

    println!("minimum element in array is:{:?}",min);

    
}