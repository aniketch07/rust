enum Light{
    Bright,
    Dull,
}

fn displayLight(light: &Light){
    match light {
        Light::Bright => println!("Bright"),
        Light::Dull => println!("Dull"),
    }
}

fn main(){
    let dull=Light::Dull;
    displayLight(&dull);
    displayLight(&dull);


}