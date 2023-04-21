enum Colors{
    red,
    yellow,
    green,
    blue,
}

fn color_name(color: Colors){
    match color{
        Colors::red =>println!("red"),
        Colors::yellow =>println!("yellow"),
        Colors::green =>println!("green"),
        Colors::blue =>println!("blue"),

    }

}

fn main(){
    color_name(Colors::red);

   

}