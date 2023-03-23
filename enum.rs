// enum Direction{
//     Up,
//     Down,
//     Left,
//     Right
// }



// fn main(){
//     let go=Direction::Up;

//     match go{
//         Direction::Up=>"Up",
//         Direction::Down=>"Down",
//         Direction::Left=>"Left",
//         Direction::Right=>"Right",
//     };

// }

enum Direction{
    Up,Down,Left,Right
}

fn which_way(go:Direction)->& 'static str{
    match go{
        Direction::Up=>"Up",
        Direction::Down=>"Down",
        Direction::Left=>"Left",
        Direction::Right=>"Right",
        
    }
}

fn main(){
    
    println!("{:?}",which_way(Direction::Up));
}