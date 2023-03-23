enum Flavor{
    Sweet,Orange,
    Sparkling,
}

struct Drink{
    flavor: Flavor,
    fluid_ounce:f64,
}

fn print_drink(drink:Drink){
    match drink.flavor{
        Flavor::Sparkling=>println!("Flavor:Sparkling"),
        Flavor::Orange=>println!("Flavor:orange"),
        Flavor::Sweet=>println!("Flavor:sweet"),
    };
}

fn main(){
    let sweet=Drink{
        flavor :Flavor::Sweet,
        fluid_ounce:6.0,
    };
    print_drink(sweet);
}