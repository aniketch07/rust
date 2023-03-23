struct Rectangle{
    height:u32,
    width:u32
}

impl Rectangle{
    fn printRect(&self){
        println!("Rectangle:{}x{}", self.height, self.width);
    }

    fn isSquare(&self) -> bool{
        self.height == self.width
    }
}

fn main(){
    let rect=Rectangle{width:10,height:10};

    rect.printRect();
   println!("{}",rect.isSquare()) ;
}