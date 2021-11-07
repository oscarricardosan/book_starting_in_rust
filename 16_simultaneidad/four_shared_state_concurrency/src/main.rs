use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let mut index= 0;
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        index += 1;
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            println!("** Ejecución de hilo {}", index);
            *num += 1;
        });
        handles.push(handle);
        println!("* Activación de hilos {}", index);
    }

    let mut index2= 0;
    for handle in handles {
        handle.join().unwrap();
        index2 += 1;
        println!("*** Finalización de hilos {}", index2);
    }

    println!("Result: {}", *counter.lock().unwrap());
}