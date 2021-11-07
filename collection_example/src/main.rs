// fn main() {
//     let v0: Vec<i32> = Vec::new();//Specifying type of data
//
//     let mut v = vec![1, 2, 3];//Rust determine the data type
//
//     {
//         let v = vec![0, 88];
//         println!("{:?}", v); // do stuff with v
//     } // <- v goes out of scope and is freed here
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
//
//     println!("{:?}", v);
//
//     let v = vec![1, 2, 3, 4, 5];
//
//     let third: &i32 = &v[2];
//     println!("The third element is {}", third);
//
//     println!("{:?}", v.get(2));
//     match v.get(2) {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("There is no third element."),
//     }
//
//     let v = vec![1, 2, 3, 4, 5];
//
//     // let does_not_exist = &v[100]; generate error
//     let does_not_exist = v.get(100);
//     println!("{:?}", does_not_exist);
//
//     let mut v = vec![1, 2, 3, 4, 5];
//     let first = &v[0];
//     // v.push(6); Genrate error to mut reference
//     println!("The first element is: {}", first);
//
//     let v = vec![100, 32, 57];
//     for i in &v {
//         println!("{}", i);
//     }
//
//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         // *i += 50; el asterisco es para desreferencia del elemento
//         *i = *i + 50;
//     }
//     println!("{:#?}", v);
//
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }
//
//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
//
//
// }

//String
// fn main() {
//
//     let data= 4;
//     let s = data.to_string();
//     println!("{}", s);
//
//     // the method also works on a literal directly:
//     let s = "initial contents".to_string();
//     //Lo anterior es igual a:
//     let s = String::from("initial contents");
//
//     //Different lenguages
//     let hello = String::from("السلام عليكم");
//     println!("{}", hello);
//     let hello = String::from("Dobrý den");
//     let hello = String::from("Hello");
//     let hello = String::from("שָׁלוֹם");
//     let hello = String::from("नमस्ते");
//     let hello = String::from("こんにちは");
//     let hello = String::from("안녕하세요");
//     let hello = String::from("你好");
//     let hello = String::from("Olá");
//     let hello = String::from("Здравствуйте");
//     let hello = String::from("Hola");
//
//     let mut s = String::from("foo");
//     s.push_str("bar");
//     s= s+"s";
//     println!("{}", s);
//
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(s2);
//     println!("s1 is {}", s1);
//     println!("s2 is {}", s2);
//
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
//
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");
//     let s = s1 + "-" + &s2 + "-" + &s3;
//     println!("{}", s);
//     // println!("s1 is {}", s1);
//
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");
//
//     //same way as println!, instead of printing to the screen, returns a String.
//     // is much easier to read and doesn’t take ownership
//     let s = format!("{}-{}-{}", s1, s2, s3);
//     println!("{}", s);
//
//     let hello = "Здравствуйте";
//     let s = &hello[0..4];
//     println!("{}", s);
//
//
//     for c in "नमस्ते".chars() {
//         println!("{}", c);
//     }
//
//     for b in "नमस्ते".bytes() {
//         println!("{}", b);
//     }
//
// }


//Hash Maps
fn main() {

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{:?}", map);


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);//hace insert porque el key yellow no existe
    scores.entry(String::from("Blue")).or_insert(50);//no hace insert porque el key blue ya existe
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}