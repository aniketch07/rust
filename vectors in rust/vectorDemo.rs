fn main() {

    let mut vect:Vec<i32> = Vec::new();

    let mut vec1=Vec::new();

    let mut vec2:Vec<String>=Vec::new();

    vec1.push(10);
    vec1.push(11);
    vec1.push(12);
    vec1.push(13);
    vec1.push(14);
for i in 0..vec1.len() {
    println!("{}", vec1[i]);
}

for i in vec1.iter() {
    println!("{}", i);
}

    //vect.push(15);
   // vec2.push(String::from("Aniket"));
  // vec2.push("Aniket");
    // println!("{}",vec1[0]);
    // println!("{}",vect[0]);
    // println!("{}",vec2[0]);

}   