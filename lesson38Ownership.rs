struct Book{
    pages:i32,
    author:String,
}

fn displyPages(book:&Book){
    println!("pages: {}", book.pages);
}

fn displayAuthor(book:&Book){
    println!("author: {}", book.author);
}

fn main(){
    let book = Book{
        pages:10,
        author: "John".to_string(),
    };
    displyPages(&book);
    displayAuthor(&book);
}