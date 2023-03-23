#![allow(unused)]  // used to get rid of warnings 

use std::io;  //usind standard input / output library
use rand::Rng; //random numbers lib

fn main(){
    let a=12;
    if even_or_odd(14){
        println!("{:?}","Even");
    }
    else{
        println!("{:?}","Odd");
    }
}

fn even_or_odd(a:i32)->bool{

    if a%2==0{
        return true;
    }
    else{
        return false;
    }
}