use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    //move le indica a rust que tome poseción de las variales
    //usadas dentro del closure.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}