//https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variable-scope
// fn main() {
//
//     // let s1 = String::from("hello");
//     // let s2 = s1;
//     //
//     // println!("{}, world!", s1);//Error ya que al hacer una copia de s1 a s2, s1 se livera de memoría del head y ya no es valida
//
//
//     let s1 = String::from("hello");
//     let s2 = s1.clone(); //no genera error ya que realiza una copia profunda de los objetos
//
//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// fn main() {
//     let mut s = String::from("hello");
//
//     let mut s2 = String::from("hello - ");
//
//     change(&mut s);
//     change(&mut s2);
//     print!("{} {} \n\n", s, s2);
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     // let mut s = String::from("hello");
//     //
//     // let r1 = &s; // no problem
//     // let r2 = &s; // no problem
//     // let r3 = &mut s; // BIG PROBLEM
//     //
//     // println!("{}, {}, and {}", r1, r2, r3);
//
//     let mut s = String::from("hello");
//
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // r1 and r2 are no longer used after this point
//
//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }

// fn main() {
//     let reference_to_nothing = no_dangle();
// }
//
// fn dangle() -> &String { // dangle returns a reference to a String
//
//     let s = String::from("hello"); // s is a new String
//
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// // Danger!
//
// fn no_dangle() -> String {
//     let s = String::from("hello");
//
//     s
// }
//This works

use std::str;

// fn main() {
//     let s = String::from("hello world A15"); // s is a new String
//     let f= first_word(&s);
//     print!("{}<- \n", f);
//     print!("{}<- \n", second_word(&s));
//
//     let s = String::from("hello");
//
//     let len = s.len();
//     let slice = &s[3..len];
//     let slice = &s[3..];
// }

// fn first_word(s: &String) -> usize {
//     let sparkle_heart = vec![104];
//     // print!("{:?}", str::from_utf8(&sparkle_heart).unwrap());
//
//     let bytes = s.as_bytes();
//     // print!("{:?}" , bytes.iter().enumerate());
//     // print!("\n");
//     for (i, &item) in bytes.iter().enumerate() {
//         // print!("{:?}" , item );//Código ascii
//         // print!("\n");
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    let mut num_word:i8= 0;
    let mut index_end_word:usize= 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if(num_word == 1){
                return &s[index_end_word..i];
            }
            index_end_word= i + 1;
            num_word= num_word+1;
        }
    }

    &s[index_end_word ..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    print!("{} \n", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    print!("{} \n", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    print!("{} \n", &word[2 ..]);
}