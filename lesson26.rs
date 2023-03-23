enum Color{
    Red,Black,Green,Pink
}

fn which_color(my_color:Color){
    match my_color{
        Color::Red=>println!("Red"),
        Color::Black=>println!("Black"),
        Color::Green=>println!("Green"),
        Color::Pink=>println!("Pink"),
    }
}

fn main(){
    
    which_color(Color::Red);
}