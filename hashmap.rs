use std::collections::HashMap;

fn main() {

    let mut map =HashMap::new();

    map.insert(1,"Aniket");
    map.insert(2,"Madhav");
    map.insert(3,"sanjay");
    map.insert(4,"raghav");
    map.insert(5,"aditi");
    map.insert(6,"stuti");

    println!("{}",map.contains_key(&9));
    println!("{}",map.len());

    map.remove(5);

    for (key,value) in map.iter() {
        println!("key:{} Value:{}",key,value);

    }
}
