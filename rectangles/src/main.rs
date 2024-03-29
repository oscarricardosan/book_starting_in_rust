#[derive(Debug)]

//https://doc.rust-lang.org/book/ch05-02-example-structs.html
// Single variables
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
//
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }


// Refactoring with Tuples
// fn main() {
//     let rect1 = (30, 50);
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
//
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


//Refactoring with Structs: Adding More Meaning
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!("rect1 is");
//     println!("{:#?}", rect1);
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }
//
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }



//Defining Methods
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }



//Methods with More Parameters

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //Methods, they use self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle_2: &Rectangle) -> bool {
        self.width > rectangle_2.width &&  self.height > rectangle_2.height
    }

    fn square(size: u32) -> Rectangle { //Associated Functions
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn new(height: u32, width: u32) -> Rectangle { //Associated Functions
        Rectangle {width, height}
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("{:#?}", sq);

    let sq = Rectangle::new(3, 5);
    println!("{:#?}", sq);
}

//
//    Yopu can define multiple impl blocks
//

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
//
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
