struct CustomerAge {
    age: i32,
}

fn can_make_purchase(customer: &CustomerAge) -> Result<bool, &'static str> {
    if customer.age > 21 {
        Ok(true)
    } else {
        Ok(false)
        //Err("Customer is not old enough to make a purchase.")
    }
}

fn main() {
    let cus1 = CustomerAge { age: 20 };
    let can_make_purchase = can_make_purchase(&cus1);
    match can_make_purchase {
        Ok(true) => println!("Purchase successful"),
        Ok(false) => println!("Purchase failed"),
        Err(e) => println!("{}", e),
    }
}
