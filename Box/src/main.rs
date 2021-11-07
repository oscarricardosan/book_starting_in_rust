use crate::MyList::{Cons, Nil};

enum MyList {
    Cons(i32, Box<MyList>),
    Nil,
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list =
        Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

}
