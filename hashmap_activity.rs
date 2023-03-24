use std::collections::HashMap;

fn main() {
    let mut stock=HashMap::new();

    stock.insert("Chair",5);
    stock.insert("Bed",3);
    stock.insert("Couches",0);
    stock.insert("Table",2);
    
    let mut totalItems=0;
    for (name,i) in stock.iter() {
        totalItems=totalItems+i;
        if i == &0 {
            
                println!("There is no stock in the database for {}",name)
        
            
        }
        else{
            println!("stock for :{} is {}",name,i);
        }

       
    }

    

    
    

    println!("Total items: {:?}", totalItems);
}