struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointers _c created.");
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(_c);
    println!("CustomSmartPointers _d created.");
}
