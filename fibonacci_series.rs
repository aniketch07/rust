fn main(){
    let mut one =0;
    let mut two=1;

    let n=5;
    print!("0,1,");
    for i in 0..n-2{
        let temp=one;
        one=one+two;
        two=one;
        print!("{:?},",one);
    }
}